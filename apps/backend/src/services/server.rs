use crate::config::SERVER_CONFIG_FILE_NAME;
use crate::core::server::config::ServerConfig;
use crate::services::binary::BinaryService;
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
	configs: HashMap<Uuid, ServerConfig>,
	processes: Arc<RwLock<HashMap<Uuid, Child>>>,
	binary_service: Arc<BinaryService>,
}

impl ServerService {
	pub fn new(path: PathBuf, binary_service: Arc<BinaryService>) -> Self {
		info!("Loading server instances");

		if !path.exists() {
			std::fs::create_dir_all(&path).expect("Failed to create server instances directory");
		}

		if !path.is_dir() {
			panic!("Server instances path must be a directory");
		}

		let dir_entries =
			std::fs::read_dir(&path).expect("Failed to read server instances directory");

		let mut configs = HashMap::<Uuid, ServerConfig>::new();

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
			configs,
			processes: Arc::new(RwLock::new(HashMap::new())),
			binary_service,
		}
	}

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

		let command_with_newline = format!("{}\n", command);

		if let Err(err) = stdin.write_all(command_with_newline.as_bytes()).await {
			return Err(ServerError::CommandError(err.to_string()));
		}

		if let Err(err) = stdin.flush().await {
			return Err(ServerError::CommandError(err.to_string()));
		}

		Ok(())
	}

	pub async fn start(&mut self, server_id: Uuid) -> Result<(), ServerError> {
		let server_config = match self.configs.get(&server_id) {
			Some(config) => config,
			None => return Err(ServerError::NoSuchServer(server_id)),
		};

		let mut processes_guard = self.processes.write().await;
		if processes_guard.contains_key(&server_id) {
			return Err(ServerError::AlreadyRunning);
		}

		// TODO: Figure out command to run
		let mut cmd = Command::new("java");

		// TODO: Set current dir to server directory
		//cmd.current_dir();
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

	pub async fn is_running(&self, server_id: Uuid) -> bool {
		let processes_guard = self.processes.read().await;
		processes_guard.contains_key(&server_id)
	}
}
