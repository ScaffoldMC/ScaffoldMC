use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS, Serialize)]
#[ts(export)]
pub struct UserResponse {
	pub id: Uuid,
	pub fullname: String,
	pub username: String,
}
