use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	pub id: Uuid,
	pub fullname: String,
	pub username: String,
	pub password_hash: String,
}

impl std::fmt::Debug for User {
	#![allow(clippy::missing_fields_in_debug)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Manually implement Debug to avoid printing password_hash
		f.debug_struct("User")
			.field("id", &self.id)
			.field("fullname", &self.fullname)
			.field("username", &self.username)
			.finish()
	}
}
