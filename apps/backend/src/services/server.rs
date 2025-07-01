use crate::server::instance::ServerInstance;
use log::{error, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct ServerService {
	instances: Arc<RwLock<HashMap<Uuid, ServerInstance>>>,
}

impl ServerService {
	pub fn new(path: PathBuf) -> Self {
		let mut instances = HashMap::new();

		info!("Loading server instances");

		if !path.exists() {
			std::fs::create_dir_all(&path).expect("Failed to create server instances directory");
		}

		if !path.is_dir() {
			panic!("Server instances path must be a directory");
		}

		let dir_entries =
			std::fs::read_dir(&path).expect("Failed to read server instances directory");

		for entry in dir_entries.flatten() {
			ServerInstance::load_from_dir(entry.path())
				.map(|instance| {
					instances.insert(instance.id, instance);
				})
				.unwrap_or_else(|err| {
					error!(
						"Failed to import server instance from {:?}: {}",
						entry.path(),
						err
					);
				});
		}

		Self {
			instances: Arc::new(RwLock::new(instances)),
		}
	}
}
