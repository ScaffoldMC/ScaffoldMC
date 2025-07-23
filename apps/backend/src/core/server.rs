use std::sync::Arc;
use tokio::{process::Child, sync::RwLock};

use crate::core::config::ServerConfig;

/// Utility struct to represent a server instance.
/// It contains the server configuration and the process handle.
pub struct Server {
	pub config: Arc<RwLock<ServerConfig>>,
	pub process: Arc<RwLock<Option<Child>>>,
}
