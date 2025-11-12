use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Clone, Serialize)]
#[ts(export)]
pub struct SudoCheckResponse {
	pub sudo: bool,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct LoginRequest {
	pub username: String,
	pub password: String,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct SudoRequest {
	pub password: String,
}
