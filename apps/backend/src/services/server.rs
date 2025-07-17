use crate::config;
use crate::config::SERVER_CONFIG_FILE_NAME;
use crate::core::server::config::ServerConfig;
use crate::services::binary::BinaryService;
use crate::services::Service;
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
	NoSuchServer(Uuid),
}

pub struct ServerService {
	servers_dir: String,
	configs: HashMap<Uuid, ServerConfig>,
	processes: Arc<RwLock<HashMap<Uuid, Child>>>,
	binary_service: Arc<BinaryService>,
}

impl Service for ServerService {
	async fn shutdown(&mut self) -> Result<(), String> {
		let server_ids: Vec<Uuid> = {
			let processes_guard = self.processes.read().await;
			processes_guard.keys().cloned().collect()
		};

		// Gracefully stop all running servers
		// TODO: Stop servers in parallel
		for server_id in server_ids {
			self.stop(server_id)
				.await
				.map_err(|e| format!("Failed to stop server {}: {}", server_id, e))?;
		}

		let mut processes_guard = self.processes.write().await;
		processes_guard.clear();

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

		let mut configs = HashMap::<Uuid, ServerConfig>::new();

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

			configs.insert(uuid, server_config);
		}

		Self {
			servers_dir,
			configs,
			processes: Arc::new(RwLock::new(HashMap::new())),
			binary_service,
		}
	}

	/// Send a command to a running server instance.
	pub async fn send_command(
		&mut self,
		server_id: Uuid,
		command: &str,
	) -> Result<(), ServerError> {
		if !self.configs.contains_key(&server_id) {
			return Err(ServerError::NoSuchServer(server_id));
		}

		let mut processes_guard = self.processes.write().await;

		let child: &mut Child = processes_guard
			.get_mut(&server_id)
			.ok_or(ServerError::NotRunning)?;

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

	/// Starts a server instance its configuration
	pub async fn start(&mut self, server_id: Uuid) -> Result<(), ServerError> {
		let server_config = match self.configs.get(&server_id) {
			Some(config) => config,
			None => return Err(ServerError::NoSuchServer(server_id)),
		};

		let mut processes_guard = self.processes.write().await;
		if processes_guard.contains_key(&server_id) {
			return Err(ServerError::AlreadyRunning);
		}

		let binary_path = self
			.binary_service
			.ensure_binary(&server_config.game)
			.await
			.map_err(|e| ServerError::StartError(e.to_string()))?;

		let mut cmd = Command::new(binary_path);

		cmd.current_dir(format!("{}/server/{}/", &self.servers_dir, server_id));
		cmd.stdin(std::process::Stdio::piped());
		cmd.stdout(std::process::Stdio::piped());
		cmd.stderr(std::process::Stdio::piped());

		match cmd.spawn() {
			Ok(child) => {
				processes_guard.insert(server_id, child);
				Ok(())
			}
			Err(e) => Err(ServerError::StartError(e.to_string())),
		}
	}

	/// Stops a running server instance.
	pub async fn stop(&mut self, server_id: Uuid) -> Result<(), ServerError> {
		let stop_command = match self.configs.get(&server_id) {
			Some(config) => config.stop_command.clone(),
			None => return Err(ServerError::NoSuchServer(server_id)),
		};

		self.send_command(server_id, &stop_command).await?;

		Ok(())
	}

	/// Checks if a server instance is currently running.
	pub async fn is_running(&self, server_id: Uuid) -> bool {
		let processes_guard = self.processes.read().await;
		processes_guard.contains_key(&server_id)
	}

	/// Creates a new server instance with the given configuration.
	pub async fn create(&mut self, server_config: ServerConfig) -> Result<Uuid, String> {
		let server_id = Uuid::new_v4();
		let server_dir = PathBuf::from(format!("{}/{}", &self.servers_dir, server_id));

		if !server_dir.exists() {
			std::fs::create_dir_all(&server_dir).map_err(|e| e.to_string())?;
		}

		let config_path = server_dir.join(SERVER_CONFIG_FILE_NAME);

		server_config
			.save_to_file(config_path)
			.map_err(|e| e.to_string())?;

		self.configs.insert(server_id, server_config);
		Ok(server_id)
	}
}
