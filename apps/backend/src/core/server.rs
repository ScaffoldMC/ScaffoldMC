use tokio::{process::Child, sync::RwLock};
use uuid::Uuid;

use crate::core::{files::server_config::ServerConfig, game::Game};

/// Server instance representation
pub struct Server {
	pub id: Uuid,
	pub config: RwLock<ServerConfig>,
	pub process: RwLock<ServerProcessState>,
}

/// Server process state
/// TODO: Add more states (starting, stopping, etc.) to give more granular info
pub enum ServerProcessState {
	Stopped,
	Running(Child),
}

/// Information about a server instance for listing purposes
pub struct ServerInfo {
	pub id: Uuid,
	pub name: String,
	pub game: Game,
}

impl Server {
	pub fn info(&self) -> ServerInfo {
		let config = self.config.blocking_read();
		let name = config.name.clone();
		let game = config.game.clone();

		ServerInfo {
			id: self.id,
			name,
			game,
		}
	}
}
