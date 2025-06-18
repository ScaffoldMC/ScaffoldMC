use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

use super::Database;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	pub id: Uuid,
	pub fullname: String,
	pub username: String,
	pub password_hash: String,
}

impl std::fmt::Debug for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Manually implement Debug to avoid printing password_hash
		f.debug_struct("User")
			.field("id", &self.id)
			.field("fullname", &self.fullname)
			.field("username", &self.username)
			.finish()
	}
}

impl Database {
	pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, sqlx::Error> {
		let user = sqlx::query_as!(
			User,
			r#"SELECT id as "id: uuid::Uuid", fullname, username, password_hash FROM users WHERE id = ?"#,
			user_id
		)
		.fetch_one(&self.pool)
		.await;

		return user;
	}

	pub async fn get_user_by_username(&self, username: &str) -> Result<User, sqlx::Error> {
		let user = sqlx::query_as!(
			User, 
			r#"SELECT id as "id: uuid::Uuid", fullname, username, password_hash FROM users WHERE username = ?"#,
			username
		)
		.fetch_one(&self.pool)
		.await;
		return user;
	}

	pub async fn update_user_username(&self, user_id: Uuid, username: &str) -> Result<(), sqlx::Error> {
		sqlx::query!(
			r#"UPDATE users SET username = ? WHERE id = ?"#,
			username,
			user_id
		)
		.execute(&self.pool)
		.await?;
		
		Ok(())
	}

	pub async fn update_user_fullname(&self, user_id: Uuid, fullname: &str) -> Result<(), sqlx::Error> {
		sqlx::query!(
			r#"UPDATE users SET fullname = ? WHERE id = ?"#,
			fullname,
			user_id
		)
		.execute(&self.pool)
		.await?;
		
		Ok(())
	}
}
