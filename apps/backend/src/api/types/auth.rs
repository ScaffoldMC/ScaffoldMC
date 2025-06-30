use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct LoginRequest {
	pub username: String,
	pub password: String,
}
