use crate::config;
use crate::config::SERVER_CONFIG_FILE_NAME;
use crate::core::bin_providers::DownloadDependency;
use crate::models::files::server_config::ServerConfig;
use crate::models::game::Game;
use crate::models::server::Server;
use crate::models::server::ServerProcessState;
use crate::models::server::ServerStateInfo;
use crate::services::binary::BinaryService;
use crate::services::Service;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ServerServiceError {
	#[error("No such server: {0}")]
	NoSuchServer(String),
	#[error("Failed to delete server: {0}")]
	DeleteError(String),
}

pub struct ServerService {
	servers: RwLock<HashMap<Uuid, Arc<Server>>>,
	binary_service: Arc<BinaryService>,
}

impl Service for ServerService {
	#[instrument(name = "ServerService.Shutdown", skip_all)]
	async fn shutdown(&mut self) -> Result<(), String> {
		let server_map = self.servers.read().await;
		let servers = server_map.values();

		// Gracefully stop all running servers
		for server in servers.clone() {
			server
				.stop()
				.await
				.map_err(|e| format!("Failed to stop a server while shutting down: {e}"))?;
		}

		// Wait 1 minute for all servers to shut down
		let timeout = std::time::Duration::from_mins(1);
		let start = std::time::Instant::now();

		for server in servers.clone() {
			while let Ok(ServerStateInfo::Running) = server.get_server_state().await {
				if start.elapsed() > timeout {
					break;
				}
				tokio::time::sleep(std::time::Duration::from_millis(100)).await;
			}
		}

		// If any servers are still running after the timeout, force kill them
		for server in servers {
			if let Ok(ServerStateInfo::Running) = server.get_server_state().await {
				if let Err(e) = server.kill().await {
					tracing::error!("Failed to force kill a server while shutting down: {}", e);
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

		let path = PathBuf::from(config::SERVERS_DIRECTORY.clone());

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
			servers: RwLock::new(servers),
			binary_service,
		}
	}

	/// Lists all server instance IDs.
	pub async fn list_server_ids(&self) -> Vec<Uuid> {
		let servers_guard = self.servers.read().await;

		servers_guard.keys().copied().collect()
	}

	/// Get a server by its ID.
	pub async fn get_server(&self, server_id: Uuid) -> Result<Arc<Server>, ServerServiceError> {
		let servers_guard = self.servers.read().await;
		let server = servers_guard
			.get(&server_id)
			.ok_or(ServerServiceError::NoSuchServer(server_id.to_string()))?;

		Ok(server.clone())
	}

	/// Creates a new server instance with the given configuration.
	#[instrument(name = "ServerService.CreateServer", skip(self))]
	pub async fn create(&self, name: &str, server_type: Game) -> Result<Uuid, String> {
		let server_id = Uuid::new_v4();
		let server_dir = config::canonical_server_dir(server_id);

		if !server_dir.exists() {
			std::fs::create_dir_all(&server_dir).map_err(|e| e.to_string())?;
		}

		let config_path = server_dir.join(SERVER_CONFIG_FILE_NAME);

		let install_result = self.binary_service.install_game(&server_type).await;

		if let Err(err) = install_result {
			return Err(format!("Failed to install game: {err}"));
		}

		let bin_info = self.binary_service.get_bin_info(&server_type).await?;

		// Extract dependency info
		let mut java_args: Vec<String> = vec![];

		for dep in bin_info.dependencies {
			match dep {
				DownloadDependency::Java(java_dependency) => {
					if let Some(args) = java_dependency.args {
						java_args = args;
					}
				}
			}
		}

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

	/// Deletes a server and removes its files
	#[instrument(name = "ServerService.DeleteServer", skip(self))]
	pub async fn delete(&self, server_id: Uuid) -> Result<(), ServerServiceError> {
		tracing::info!("Deleting server {}", server_id);
		let server = self.get_server(server_id).await?;

		// Stop the server if it's running
		if let Ok(ServerStateInfo::Running) = server.get_server_state().await {
			// FIXME: Error handling here doesn't feel right
			server
				.stop()
				.await
				.map_err(|err| ServerServiceError::DeleteError(err.to_string()))?;
		}

		let mut servers_guard = self.servers.write().await;
		servers_guard
			.remove(&server_id)
			.ok_or(ServerServiceError::NoSuchServer(server_id.to_string()))?;

		let server_dir = config::canonical_server_dir(server_id);
		std::fs::remove_dir_all(&server_dir).map_err(|e| {
			ServerServiceError::DeleteError(format!("Failed to delete server files: {e}"))
		})?;

		tracing::info!("Server {} deleted successfully", server_id);

		Ok(())
	}
}
