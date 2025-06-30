use crate::server::instance::ServerInstance;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct ServerService {
	instances: Arc<RwLock<HashMap<Uuid, ServerInstance>>>,
}

impl ServerService {
	pub fn new() -> Self {
		Self {
			instances: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	// TODO: Business logic methods for managing server instances
}
