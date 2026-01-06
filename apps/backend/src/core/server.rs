use serde::{Deserialize, Serialize};
use tokio::{process::Child, sync::RwLock};
use ts_rs::TS;
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

impl ServerProcessState {
	pub fn info(&self) -> ServerStateInfo {
		match self {
			ServerProcessState::Stopped => ServerStateInfo::Stopped,
			ServerProcessState::Running(_) => ServerStateInfo::Running,
		}
	}
}

/// Information about the current state of a server
#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ServerStateInfo {
	Stopped,
	Running,
}

/// Information about a server instance for listing purposes
#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ServerInfo {
	pub id: Uuid,
	pub name: String,
	pub game: Game,
	pub state: ServerStateInfo,
}

impl Server {
	pub async fn info(&self) -> ServerInfo {
		let config = self.config.read().await;
		let name = config.name.clone();
		let game = config.game.clone();
		let state = self.process.read().await.info();

		ServerInfo {
			id: self.id,
			name,
			game,
			state,
		}
	}
}
