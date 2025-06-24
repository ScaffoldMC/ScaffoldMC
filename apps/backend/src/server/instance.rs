use std::path::PathBuf;

use tokio::process::Child;
use uuid::Uuid;

use crate::{config::SERVER_CONFIG_FILE_NAME, server::config::ServerConfig};

pub struct ServerInstance {
	id: Uuid,
	pub config: ServerConfig,
	process: Option<Child>,
}

impl ServerInstance {
	pub fn new(config: ServerConfig) -> Self {
		Self {
			id: Uuid::new_v4(),
			config,
			process: None,
		}
	}

	pub fn load_from_dir(path: PathBuf) -> Result<Self, String> {
		if !path.is_dir() {
			return Err(format!("Path {:?} is not a directory", path));
		}

		let config_path = path.join(SERVER_CONFIG_FILE_NAME);
		let server_config = ServerConfig::load_from_file(config_path);

		match server_config {
			Ok(server_config) => Ok(Self::new(server_config)),
			Err(e) => Err(format!("Failed to load server config: {}", e)),
		}
	}
}
