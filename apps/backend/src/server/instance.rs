use tokio::process::Child;
use uuid::Uuid;

use crate::server::config::ServerConfig;

pub struct ServerInstance {
	id: Uuid,
	config: ServerConfig,
	process: Option<Child>,
}
