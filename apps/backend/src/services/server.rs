use crate::config;
use crate::config::SERVER_CONFIG_FILE_NAME;
use crate::core::bin_providers::DownloadInfo;
use crate::core::files::server_config::ServerConfig;
use crate::core::game::Game;
use crate::core::server::Server;
use crate::core::server::ServerInfo;
use crate::core::server::ServerProcessState;
use crate::services::binary::BinaryService;
use crate::services::Service;
use futures_util::future::join_all;
use log::{error, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::process::Child;
use tokio::process::Command;
use tokio::sync::RwLock;
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
	async fn shutdown(&mut self) -> Result<(), String> {
		let server_ids: Vec<Uuid> = {
			let servers_guard = self.servers.read().await;
			servers_guard.keys().cloned().collect()
		};

		// Gracefully stop all running servers
		for server_id in server_ids.clone() {
			self.stop(server_id)
				.await
				.map_err(|e| format!("Failed to stop server {}: {}", server_id, e))?;
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
					error!("Failed to force kill server {}: {}", server_id, e);
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
	pub fn new(binary_service: Arc<BinaryService>) -> Self {
		info!("Loading server instances");

		let servers_dir = format!("{}/servers", config::DATA_FOLDER);
		let path = PathBuf::from(&servers_dir);

		if !path.exists() {
			std::fs::create_dir_all(&path).expect("Failed to create server instances directory");
		}

		if !path.is_dir() {
			panic!("Server instances path must be a directory");
		}

		let dir_entries =
			std::fs::read_dir(&path).expect("Failed to read server instances directory");

		let mut servers = HashMap::<Uuid, Arc<Server>>::new();

		// Load the server configurations from the server instances directory.
		for entry in dir_entries {
			if let Err(err) = entry {
				error!("Failed to read directory entry: {}", err);
				continue;
			}

			let entry = entry.unwrap();

			if !path.is_dir() {
				error!("Path {:?} is not a directory", path);
				continue;
			}

			let path = entry.path();

			let dir_name = path.file_name().and_then(|name| name.to_str());

			let dir_name = match dir_name {
				Some(name) => name,
				None => {
					error!("Failed to get directory name from path {:?}", path);
					continue;
				}
			};

			let uuid = match Uuid::try_parse(dir_name) {
				Ok(uuid) => uuid,
				Err(_) => {
					error!("Invalid UUID in directory name: {}", dir_name);
					continue;
				}
			};

			let config_path = path.join(SERVER_CONFIG_FILE_NAME);

			let server_config = match ServerConfig::load_from_file(config_path.clone()) {
				Ok(cfg) => cfg,
				Err(e) => {
					error!("Failed to load server config from {:?}: {}", config_path, e);
					continue;
				}
			};

			let server = Arc::new(Server {
				id: uuid,
				config: RwLock::new(server_config),
				process: RwLock::new(ServerProcessState::Stopped),
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

		servers_guard.keys().cloned().collect()
	}

	/// Gets information about a server instance by ID.
	pub async fn get_server_info(&self, server_id: Uuid) -> Result<ServerInfo, ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let info = server.info().await;

		Ok(info)
	}

	/// Send a command to a running server instance.
	pub async fn send_command(&self, server_id: Uuid, command: &str) -> Result<(), ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let mut process_guard = server.process.write().await;
		let child: &mut Child = match &mut *process_guard {
			ServerProcessState::Running(child) => child,
			_ => return Err(ServerError::NotRunning),
		};

		let stdin = child.stdin.as_mut().ok_or(ServerError::NotRunning)?;

		// Ensure the command ends with a newline so it is actually sent
		// instead of being buffered and screwing up future commands.
		let command_with_newline = if command.ends_with('\n') {
			command.to_string()
		} else {
			format!("{command}\n")
		};

		if let Err(err) = stdin.write_all(command_with_newline.as_bytes()).await {
			return Err(ServerError::CommandError(err.to_string()));
		}

		if let Err(err) = stdin.flush().await {
			return Err(ServerError::CommandError(err.to_string()));
		}

		Ok(())
	}

	/// Starts a server instance by ID using its configuration.
	pub async fn start(&self, server_id: Uuid) -> Result<(), ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let mut process_guard = server.process.write().await;

		if let ServerProcessState::Running(_) = *process_guard {
			return Err(ServerError::AlreadyRunning);
		}

		let config_guard = server.config.read().await;
		let binary_path = self
			.binary_service
			.ensure_binary(&config_guard.game)
			.await
			.map_err(|e| ServerError::StartError(e.to_string()))?;

		let mut cmd = Command::new(binary_path);

		cmd.current_dir(format!("{}/server/{}/", &self.servers_dir, server_id));
		cmd.stdin(std::process::Stdio::piped());
		cmd.stdout(std::process::Stdio::piped());
		cmd.stderr(std::process::Stdio::piped());

		match cmd.spawn() {
			Ok(child) => {
				*process_guard = ServerProcessState::Running(child);
				Ok(())
			}
			Err(e) => Err(ServerError::StartError(e.to_string())),
		}
	}

	/// Stops a running server instance.
	pub async fn stop(&self, server_id: Uuid) -> Result<(), ServerError> {
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

	pub async fn kill(&self, server_id: Uuid) -> Result<(), ServerError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerError::NoSuchServer(server_id.to_string()))?;

		let mut process_guard = server.process.write().await;

		if let ServerProcessState::Running(child) = &mut *process_guard {
			if let Err(e) = child.kill().await {
				return Err(ServerError::StopError(e.to_string()));
			}
		} else {
			return Err(ServerError::NotRunning);
		}

		Ok(())
	}

	/// Checks if a server instance is currently running.
	pub async fn is_running(&self, server_id: Uuid) -> Result<bool, ServerError> {
		let servers_guard = self.servers.read().await;
		if let Some(server) = servers_guard.get(&server_id) {
			let process_guard = server.process.read().await;
			match *process_guard {
				ServerProcessState::Running(_) => Ok(true),
				_ => Ok(false),
			}
		} else {
			Err(ServerError::NoSuchServer(server_id.to_string()))
		}
	}

	/// Creates a new server instance with the given configuration.
	pub async fn create(&self, name: &str, server_type: Game) -> Result<Uuid, String> {
		let server_id = Uuid::new_v4();
		let server_dir = PathBuf::from(format!("{}/{}", &self.servers_dir, server_id));

		if !server_dir.exists() {
			std::fs::create_dir_all(&server_dir).map_err(|e| e.to_string())?;
		}

		let config_path = server_dir.join(SERVER_CONFIG_FILE_NAME);

		let install_result = self.binary_service.install_game(&server_type).await;

		if let Err(err) = install_result {
			return Err(format!("Failed to install game: {}", err));
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
		});

		let mut servers_guard = self.servers.write().await;
		servers_guard.insert(server_id, server);
		Ok(server_id)
	}
}
