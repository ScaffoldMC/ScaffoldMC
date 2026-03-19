use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::core::game::Game;

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct PartialServer {
	pub id: Uuid,
	pub name: String,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateServerRequest {
	pub name: String,
	pub game: Game,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct ServerCommandRequest {
	pub command: String,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct ConsoleQueryParams {
	pub since: Option<u64>,
}
