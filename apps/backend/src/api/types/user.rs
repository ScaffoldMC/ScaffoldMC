use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::db::user::User;

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
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
