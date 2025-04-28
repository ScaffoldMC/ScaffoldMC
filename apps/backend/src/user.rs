use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	id: Uuid,
	name: String,
	username: String,
	password_hash: String,
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

impl AuthUser for User {
	type Id = Uuid;

	fn id(&self) -> Self::Id {
		self.id
	}

	fn session_auth_hash(&self) -> &[u8] {
		// TODO: Replace with a more secure hash function
		self.password_hash.as_bytes()
	}
}
