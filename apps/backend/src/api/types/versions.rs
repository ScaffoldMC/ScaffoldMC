use crate::core::game::Game;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CompleteVersionResponse {
	pub game: Game,
}
#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct OptionsResponse<T> {
	pub message: String,
	pub options: Vec<T>,
}
