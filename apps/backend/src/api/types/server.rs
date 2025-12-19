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
}
