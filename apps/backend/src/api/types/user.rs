use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::db::user::User;

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct UserPatchRequest {
	pub fullname: Option<String>,
	pub username: Option<String>,
	pub password: Option<String>,
	pub new_password: Option<String>,
}

#[derive(TS, Serialize)]
#[ts(export)]
pub struct UserResponse {
	pub id: Uuid,
	pub fullname: String,
	pub username: String,
}

impl From<User> for UserResponse {
	fn from(db_user: User) -> Self {
		UserResponse {
			id: db_user.id,
			fullname: db_user.fullname,
			username: db_user.username,
		}
	}
}
