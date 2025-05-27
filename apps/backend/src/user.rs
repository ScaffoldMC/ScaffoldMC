use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	id: Uuid,
	name: String,
	username: String,
	password_hash: String,
}

impl User {
	pub fn username(&self) -> &str {
		&self.username
	}

	pub fn password_hash(&self) -> &str {
		&self.password_hash
	}
}

impl std::fmt::Debug for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Manually implement Debug to avoid printing password_hash
		f.debug_struct("User")
			.field("id", &self.id)
			.field("name", &self.name)
			.field("username", &self.username)
			.finish()
	}
}
