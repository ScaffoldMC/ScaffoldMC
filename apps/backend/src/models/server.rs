use crate::config;
use crate::config::SERVER_CONSOLE_MAX_LINES;
use crate::config::SERVER_WATCHER_TICK;
use crate::models::files::server_config::PartialServerConfig;
use crate::models::files::server_config::ServerConfig;
use crate::models::game::Game;
use crate::services::binary::BinaryService;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
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
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ServerError {
	#[error("Server is already running")]
	AlreadyRunning,
	#[error("Failed to delete the server: {0}")]
	DeleteError(String),
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

#[derive(Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct ConsoleLine {
	pub num: u64,
	pub stream: ConsoleStreamType,
	pub line: String,
}

#[derive(Clone, Copy, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum ConsoleStreamType {
	Stdout,
	Stderr,
}

#[derive(Debug, Clone)]
pub enum ProcessCommand {
	Kill,
	Write(String),
}

pub struct ServerRuntime {
	pub command_tx: mpsc::Sender<ProcessCommand>,
	pub running_rx: watch::Receiver<bool>,
}

impl ServerRuntime {
	pub async fn kill(&self) -> Result<(), String> {
		self.command_tx
			.send(ProcessCommand::Kill)
			.await
			.map_err(|e| format!("Failed to send kill command: {e}"))?;

		Ok(())
	}

	pub async fn send_line(&self, line: String) -> Result<(), String> {
		self.command_tx
			.send(ProcessCommand::Write(line))
			.await
			.map_err(|_| "Process supervisor is not running".to_string())
	}

	pub fn is_running(&self) -> bool {
		*self.running_rx.borrow()
	}
}

/// Server process state
pub enum ServerProcessState {
	Stopped,
	Starting,
	Running(Arc<ServerRuntime>),
}

impl ServerProcessState {
	pub fn info(&self) -> ServerStateInfo {
		match self {
			ServerProcessState::Stopped => ServerStateInfo::Stopped,
			ServerProcessState::Starting => ServerStateInfo::Starting,
			ServerProcessState::Running(_) => ServerStateInfo::Running,
		}
	}
}

/// Information about the current state of a server
#[derive(Clone, Serialize, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum ServerStateInfo {
	Stopped,
	Running,
	Starting,
}

/// Information about a server instance for listing purposes
#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ServerInfo {
	pub id: Uuid,
	pub config: ServerConfig,
	pub state: ServerStateInfo,
}

/// Server instance representation
pub struct Server {
	id: Uuid,
	config: RwLock<ServerConfig>,
	process: RwLock<ServerProcessState>,
	console_lines: RwLock<VecDeque<ConsoleLine>>,
	next_line_num: AtomicU64,
}

impl Server {
	/// Get a server's ID
	pub async fn id(&self) -> Uuid {
		self.id
	}

	/// Gets information about a server instance by ID.
	#[instrument(name = "Server.GetServerInfo", skip(self))]
	pub async fn get_server_info(&self) -> ServerInfo {
		let config = self.config.read().await;
		let state = self.process.read().await.info();

		ServerInfo {
			id: self.id,
			config: config.clone(),
			state,
		}
	}

	/// Send a command to the server
	#[instrument(name = "Server.SendCommand", skip(self))]
	pub async fn send_command(&self, command: &str) -> Result<(), ServerError> {
		tracing::info!("Sending command to server: {}", command);

		let process_guard = self.process.read().await;

		match &*process_guard {
			ServerProcessState::Starting | ServerProcessState::Stopped => {
				Err(ServerError::NotRunning)
			}
			ServerProcessState::Running(runtime) => runtime
				.send_line(command.to_string())
				.await
				.map_err(ServerError::CommandError),
		}
	}

	/// Start the server instance
	#[instrument(name = "Server.StartServer", skip(self, binary_service))]
	pub async fn start(
		self: &Arc<Self>,
		binary_service: Arc<BinaryService>,
	) -> Result<(), ServerError> {
		tracing::info!("Starting server instance");

		let mut process_guard = self.process.write().await;
		if let ServerProcessState::Running(_) = *process_guard {
			return Err(ServerError::AlreadyRunning);
		}

		*process_guard = ServerProcessState::Starting;
		drop(process_guard);

		let config_guard = self.config.read().await;

		// Build absolute paths for server binary and directory
		let binary_path = binary_service
			.ensure_binary(&config_guard.game)
			.await
			.map_err(|e| ServerError::StartError(e.clone()))?;
		let binary_path = std::fs::canonicalize(&binary_path)
			.map_err(|e| ServerError::StartError(format!("Invalid binary path: {e}")))?;
		let binary_path = binary_path.to_str().ok_or_else(|| {
			ServerError::StartError("Binary path contains invalid UTF-8 characters".to_string())
		})?;

		let server_dir = config::canonical_server_dir(self.id);
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
		let mut process_guard = self.process.write().await;
		*process_guard = ServerProcessState::Running(runtime.clone());
		drop(process_guard);

		// Reset console buffer and line number
		self.next_line_num.store(0, Ordering::Relaxed);
		self.console_lines.write().await.clear();

		Self::start_watcher(
			self,
			child,
			running_tx.clone(),
			command_rx,
			stdin,
			stdout,
			stderr,
		);

		Ok(())
	}

	/// Stops the server instance.
	#[instrument(name = "Server.StopServer", skip(self))]
	pub async fn stop(&self) -> Result<(), ServerError> {
		tracing::info!("Stopping server instance.");

		let stop_command = {
			let config_guard = self.config.read().await;
			config_guard.stop_command.clone()
		};

		self.send_command(&stop_command).await?;

		Ok(())
	}

	/// Kills the server instance.
	#[instrument(name = "Server.Kill", skip(self))]
	pub async fn kill(&self) -> Result<(), ServerError> {
		tracing::info!("Killing server instance");

		let process_guard = self.process.read().await;

		match &*process_guard {
			ServerProcessState::Stopped | ServerProcessState::Starting => {
				Err(ServerError::NotRunning)
			}
			ServerProcessState::Running(runtime) => {
				runtime.kill().await.map_err(ServerError::StopError)
			}
		}
	}

	/// Gets a server's state
	pub async fn get_server_state(&self) -> Result<ServerStateInfo, ServerError> {
		let server_state = self.process.read().await;
		Ok(server_state.info())
	}

	/// Update server config
	#[instrument(name = "Server.UpdateConfig", skip(self))]
	pub async fn update_config(&self, new_config: PartialServerConfig) -> Result<(), ServerError> {
		let mut config_guard = self.config.write().await;

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

	/// Get a snapshot of the server's console output
	pub async fn get_console_snapshot(
		&self,
		since_line: Option<u64>,
	) -> Result<Vec<ConsoleLine>, ServerError> {
		let lines = self.console_lines.read().await;
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
