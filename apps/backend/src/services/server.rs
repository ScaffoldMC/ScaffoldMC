use crate::config;
use crate::config::SERVER_CONFIG_FILE_NAME;
use crate::config::SERVER_CONSOLE_MAX_LINES;
use crate::config::SERVER_WATCHER_TICK;
use crate::core::bin_providers::DownloadInfo;
use crate::core::files::server_config::PartialServerConfig;
use crate::core::files::server_config::ServerConfig;
use crate::core::game::Game;
use crate::core::server::ConsoleLine;
use crate::core::server::ConsoleStreamType;
use crate::core::server::ProcessCommand;
use crate::core::server::Server;
use crate::core::server::ServerInfo;
use crate::core::server::ServerProcessState;
use crate::core::server::ServerRuntime;
use crate::services::binary::BinaryService;
use crate::services::Service;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::process::Child;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::watch;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tracing::instrument;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ServerError {
	#[error("Server is already running")]
	AlreadyRunning,
	#[error("Failed to start server: {0}")]
	StartError(String),
	#[error("Failed to stop server: {0}")]
	StopError(String),
	#[error("Failed to send command to server: {0}")]
	CommandError(String),
	#[error("Server is not running")]
	NotRunning,
	#[error("No such server: {0}")]
	NoSuchServer(String),
}

pub struct ServerService {
	servers_dir: String,
	servers: RwLock<HashMap<Uuid, Arc<Server>>>,
	binary_service: Arc<BinaryService>,
}

impl Service for ServerService {
	#[instrument(name = "ServerService.Shutdown", skip_all)]
	async fn shutdown(&mut self) -> Result<(), String> {
		let server_ids: Vec<Uuid> = {
			let servers_guard = self.servers.read().await;
			servers_guard.keys().copied().collect()
		};

		// Gracefully stop all running servers
		for server_id in server_ids.clone() {
			self.stop(server_id)
				.await
				.map_err(|e| format!("Failed to stop server {server_id}: {e}"))?;
		}

		// Wait up to 30 seconds for all servers to shut down
		let timeout = std::time::Duration::from_secs(60);
		let start = std::time::Instant::now();

		for server_id in server_ids.clone() {
			while let Ok(true) = self.is_running(server_id).await {
				if start.elapsed() > timeout {
					break;
				}
				tokio::time::sleep(std::time::Duration::from_millis(100)).await;
			}
		}

		// If any servers are still running after the timeout, force kill them
		for server_id in server_ids {
			if let Ok(true) = self.is_running(server_id).await {
				if let Err(e) = self.kill(server_id).await {
					tracing::error!("Failed to force kill server {}: {}", server_id, e);
				}
			}
		}

		let mut servers_guard = self.servers.write().await;
		servers_guard.clear();

		Ok(())
	}
}

/// Service for managing server instances.
impl ServerService {
	/// Creates a new `ServerService` instance.
	#[instrument(name = "ServerService.Startup", skip_all)]
	pub fn new(binary_service: Arc<BinaryService>) -> Self {
		tracing::info!("Loading server instances");

		let servers_dir = format!("{}/servers", config::DATA_FOLDER);
		let path = PathBuf::from(&servers_dir);

		if !path.exists() {
			std::fs::create_dir_all(&path).expect("Failed to create server instances directory");
		}

		assert!(path.is_dir(), "Server instances path must be a directory");

		let dir_entries =
			std::fs::read_dir(&path).expect("Failed to read server instances directory");

		let mut servers = HashMap::<Uuid, Arc<Server>>::new();

		// Load the server configurations from the server instances directory.
		for entry in dir_entries {
			if let Err(err) = entry {
				tracing::error!("Failed to read directory entry: {}", err);
				continue;
			}

			let entry = entry.unwrap();

			if !path.is_dir() {
				tracing::error!("Path {:?} is not a directory", path);
				continue;
			}

			let path = entry.path();

			let dir_name = path.file_name().and_then(|name| name.to_str());

			let Some(name) = dir_name else {
				tracing::error!("Failed to get directory name from path {:?}", path);
				continue;
			};

			let Ok(uuid) = Uuid::try_parse(name) else {
				tracing::error!("Invalid UUID in directory name: {}", name);
				continue;
			};

			let config_path = path.join(SERVER_CONFIG_FILE_NAME);

			let server_config = match ServerConfig::load_from_file(config_path.clone()) {
				Ok(cfg) => cfg,
				Err(e) => {
					tracing::error!("Failed to load server config from {:?}: {}", config_path, e);
					continue;
				}
			};

			let server = Arc::new(Server {
				id: uuid,
				config: RwLock::new(server_config),
				process: RwLock::new(ServerProcessState::Stopped),
				console_lines: RwLock::new(VecDeque::new()),
				next_line_num: AtomicU64::new(0),
			});

			servers.insert(uuid, server);
		}

		Self {
			servers_dir,
			servers: RwLock::new(servers),
			binary_service,
		}
	}

	/// Lists all server instance IDs.
	pub async fn list_server_ids(&self) -> Vec<Uuid> {
		let servers_guard = self.servers.read().await;

		servers_guard.keys().copied().collect()
	}

	/// Gets information about a server instance by ID.
	#[instrument(name = "ServerService.GetServerInfo", skip(self))]
	pub async fn get_server_info(&self, server_id: Uuid) -> Result<ServerInfo, ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let info = server.info().await;

		Ok(info)
	}

	/// Send a command to a running server instance.
	#[instrument(name = "ServerService.SendCommand", skip(self))]
	pub async fn send_command(&self, server_id: Uuid, command: &str) -> Result<(), ServerError> {
		tracing::info!("Sending command to server {}: {}", server_id, command);

		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let process_guard = server.process.read().await;

		match &*process_guard {
			ServerProcessState::Stopped => Err(ServerError::NotRunning),
			ServerProcessState::Running(runtime) => runtime
				.send_line(command.to_string())
				.await
				.map_err(ServerError::CommandError),
		}
	}

	/// Starts a server instance by ID using its configuration.
	#[instrument(name = "ServerService.StartServer", skip(self))]
	pub async fn start(&self, server_id: Uuid) -> Result<(), ServerError> {
		tracing::info!("Starting server instance {}", server_id);

		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let mut process_guard = server.process.write().await;

		if let ServerProcessState::Running(_) = *process_guard {
			return Err(ServerError::AlreadyRunning);
		}

		let config_guard = server.config.read().await;

		// Build absolute paths for server binary and directory
		let binary_path = self
			.binary_service
			.ensure_binary(&config_guard.game)
			.await
			.map_err(|e| ServerError::StartError(e.clone()))?;
		let binary_path = std::fs::canonicalize(&binary_path)
			.map_err(|e| ServerError::StartError(format!("Invalid binary path: {e}")))?;
		let binary_path = binary_path.to_str().ok_or_else(|| {
			ServerError::StartError("Binary path contains invalid UTF-8 characters".to_string())
		})?;

		let server_dir = format!("{}/{}/", &self.servers_dir, server_id);
		let server_dir = std::fs::canonicalize(&server_dir)
			.map_err(|e| ServerError::StartError(format!("Invalid server directory path: {e}")))?;

		// Create command to start the server process
		let mut cmd = match config_guard.game {
			Game::MinecraftJava(_) => {
				let mut cmd = Command::new("java");

				cmd.arg("-jar");
				cmd.arg(binary_path);

				for arg in &config_guard.args {
					cmd.arg(arg);
				}

				cmd
			}
		};

		cmd.current_dir(&server_dir);
		cmd.stdin(std::process::Stdio::piped());
		cmd.stdout(std::process::Stdio::piped());
		cmd.stderr(std::process::Stdio::piped());

		// Spawn child and create runtime
		let mut child = cmd
			.spawn()
			.map_err(|e| ServerError::StartError(e.to_string()))?;

		let stdin = child.stdin.take();
		let stdout = child.stdout.take();
		let stderr = child.stderr.take();

		let (command_tx, command_rx) = mpsc::channel::<ProcessCommand>(64);
		let (running_tx, running_rx) = watch::channel(true);

		let runtime = Arc::new(ServerRuntime {
			command_tx,
			running_rx,
		});

		// Set state to running
		*process_guard = ServerProcessState::Running(runtime.clone());
		drop(process_guard);

		// Reset console buffer and line number
		server.next_line_num.store(0, Ordering::Relaxed);
		server.console_lines.write().await.clear();

		Self::start_watcher(
			server,
			child,
			running_tx.clone(),
			command_rx,
			stdin,
			stdout,
			stderr,
		);

		Ok(())
	}

	/// Stops a running server instance.
	#[instrument(name = "ServerService.StopServer", skip(self))]
	pub async fn stop(&self, server_id: Uuid) -> Result<(), ServerError> {
		tracing::info!("Stopping server instance {}", server_id);

		let stop_command = {
			let servers_guard = self.servers.read().await;
			let server = servers_guard
				.get(&server_id)
				.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;
			let config_guard = server.config.read().await;
			config_guard.stop_command.clone()
		};

		self.send_command(server_id, &stop_command).await?;

		Ok(())
	}

	#[instrument(name = "ServerService.KillServer", skip(self))]
	pub async fn kill(&self, server_id: Uuid) -> Result<(), ServerError> {
		tracing::info!("Killing server instance {}", server_id);

		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let process_guard = server.process.read().await;

		match &*process_guard {
			ServerProcessState::Stopped => Err(ServerError::NotRunning),
			ServerProcessState::Running(runtime) => {
				runtime.kill().await.map_err(ServerError::StopError)
			}
		}
	}

	/// Checks if a server instance is currently running.
	pub async fn is_running(&self, server_id: Uuid) -> Result<bool, ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let process_guard = server.process.read().await;
		match &*process_guard {
			ServerProcessState::Running(runtime) => Ok(runtime.is_running()),
			ServerProcessState::Stopped => Ok(false),
		}
	}

	/// Get a server's config
	#[instrument(name = "ServerService.GetConfig", skip(self))]
	pub async fn get_config(&self, server_id: Uuid) -> Result<ServerConfig, ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let config_guard = server.config.read().await;
		Ok(config_guard.clone())
	}

	/// Update a server's config
	#[instrument(name = "ServerService.UpdateConfig", skip(self))]
	pub async fn update_config(
		&self,
		server_id: Uuid,
		new_config: PartialServerConfig,
	) -> Result<(), ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let mut config_guard = server.config.write().await;

		if let Some(name) = new_config.name {
			config_guard.name = name;
		}

		if let Some(game) = new_config.game {
			config_guard.game = game;
		}

		if let Some(args) = new_config.args {
			config_guard.args = args;
		}

		if let Some(stop_command) = new_config.stop_command {
			config_guard.stop_command = stop_command;
		}

		Ok(())
	}

	/// Creates a new server instance with the given configuration.
	#[instrument(name = "ServerService.CreateServer", skip(self))]
	pub async fn create(&self, name: &str, server_type: Game) -> Result<Uuid, String> {
		let server_id = Uuid::new_v4();
		let server_dir = PathBuf::from(format!("{}/{}", &self.servers_dir, server_id));

		if !server_dir.exists() {
			std::fs::create_dir_all(&server_dir).map_err(|e| e.to_string())?;
		}

		let config_path = server_dir.join(SERVER_CONFIG_FILE_NAME);

		let install_result = self.binary_service.install_game(&server_type).await;

		if let Err(err) = install_result {
			return Err(format!("Failed to install game: {err}"));
		}

		let bin_info = self.binary_service.get_bin_info(&server_type).await?;

		let java_args = match &bin_info {
			DownloadInfo::MinecraftJava(info) => info.java_rec_args(),
		};

		let server_config = ServerConfig {
			name: name.to_string(),
			game: server_type,
			args: java_args,
			stop_command: "stop".into(), // TODO: Velocity uses "shutdown"
		};

		server_config
			.save_to_file(config_path)
			.map_err(|e| e.to_string())?;

		let server = Arc::new(Server {
			id: server_id,
			config: RwLock::new(server_config),
			process: RwLock::new(ServerProcessState::Stopped),
			console_lines: RwLock::new(VecDeque::new()),
			next_line_num: AtomicU64::new(0),
		});

		tracing::info!("Creating new server instance: name='{}'", name);

		let mut servers_guard = self.servers.write().await;
		servers_guard.insert(server_id, server);
		Ok(server_id)
	}

	/// Get a snapshot of a server's console output
	pub async fn get_console_snapshot(
		&self,
		server_id: Uuid,
		since_line: Option<u64>,
	) -> Result<Vec<ConsoleLine>, ServerError> {
		let servers = self.servers.read().await;
		let server = servers
			.get(&server_id)
			.cloned()
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let lines = server.console_lines.read().await;
		let iter = lines
			.iter()
			.filter(|l| since_line.is_none_or(|s| l.num > s));

		Ok(iter.take(SERVER_CONSOLE_MAX_LINES).cloned().collect())
	}

	/// Internal: Generic reader task for stdout/stderr of a server process
	fn reader_task<R: AsyncRead + Unpin + Send + 'static>(
		server: Arc<Server>,
		reader: R,
		stream: ConsoleStreamType,
	) -> JoinHandle<()> {
		tokio::spawn(async move {
			let mut lines = BufReader::new(reader).lines();
			while let Ok(Some(line)) = lines.next_line().await {
				// Push line to console buffer and increase line number
				let num = server.next_line_num.fetch_add(1, Ordering::Relaxed);
				let mut buf = server.console_lines.write().await;

				if buf.len() >= SERVER_CONSOLE_MAX_LINES {
					buf.pop_front();
				}

				buf.push_back(ConsoleLine { num, stream, line });
			}
		})
	}

	/// Internal: Spawns a watcher task for a server process.
	fn start_watcher(
		server: &Arc<Server>,
		child: Child,
		running_tx: watch::Sender<bool>,
		command_rx: mpsc::Receiver<ProcessCommand>,
		stdin: Option<tokio::process::ChildStdin>,
		stdout: Option<tokio::process::ChildStdout>,
		stderr: Option<tokio::process::ChildStderr>,
	) {
		let stdout_reader =
			Self::reader_task(server.clone(), stdout.unwrap(), ConsoleStreamType::Stdout);
		let stderr_reader =
			Self::reader_task(server.clone(), stderr.unwrap(), ConsoleStreamType::Stderr);

		let server_for_watcher = server.clone();
		let watcher = async move {
			Self::watcher_loop(child, command_rx, stdin).await;

			// End readers once the process has exited
			stdout_reader.abort();
			stderr_reader.abort();

			// Update state to stopped
			let _ = running_tx.send(false);
			let mut guard = server_for_watcher.process.write().await;
			*guard = ServerProcessState::Stopped;
		};

		tokio::spawn(watcher.instrument(
			tracing::info_span!(parent: None, "ServerWatcher", server_id = %server.id),
		));
	}

	/// Internal: Watcher loop function that handles process monitoring and command execution.
	/// Exits on process termination or on error.
	async fn watcher_loop(
		mut child: Child,
		mut command_rx: mpsc::Receiver<ProcessCommand>,
		mut stdin: Option<tokio::process::ChildStdin>,
	) {
		let mut ticker = tokio::time::interval(SERVER_WATCHER_TICK);

		loop {
			tokio::select! {
				maybe_cmd = command_rx.recv() => {
					match maybe_cmd {
						Some(ProcessCommand::Kill) => {
							if let Err(e) = child.kill().await {
								tracing::warn!("kill failed: {}", e);
							}
						},
						Some(ProcessCommand::Write(mut line)) => {
							if !line.ends_with('\n') {
								line.push('\n');
							}
							if let Some(stdin_handle) = stdin.as_mut() {
								if let Err(e) = stdin_handle.write_all(line.as_bytes()).await {
									tracing::warn!("stdin write failed: {}", e);
								} else if let Err(e) = stdin_handle.flush().await {
									tracing::warn!("stdin flush failed: {}", e);
								}
							}
						},
						None => break,
					}
				}
				_ = ticker.tick() => {
					match child.try_wait() {
						Ok(Some(status)) => {
							tracing::info!("Server process exited: {}", status);
							break;
						}
						Ok(None) => {}
						Err(e) => {
							tracing::warn!("try_wait failed: {}", e);
							break;
						}
					}
				}
			}
		}
	}
}
