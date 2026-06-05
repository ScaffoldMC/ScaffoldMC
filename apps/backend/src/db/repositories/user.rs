use crate::db::models::user::User;
use async_trait::async_trait;
use sqlx::types::Uuid;

/// Repository structure for managing users in the database.
#[async_trait]
pub trait UserRepository: Send + Sync {
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, sqlx::Error>;
	async fn get_user_by_username(&self, username: &str) -> Result<User, sqlx::Error>;
	async fn update_user_username(&self, user_id: Uuid, username: &str) -> Result<(), sqlx::Error>;
	async fn update_user_fullname(&self, user_id: Uuid, fullname: &str) -> Result<(), sqlx::Error>;
	async fn update_user_password_hash(
		&self,
		user_id: Uuid,
		password_hash: &str,
	) -> Result<(), sqlx::Error>;
	async fn create_user(
		&self,
		user_id: Uuid,
		fullname: &str,
		username: &str,
		password_hash: &str,
	) -> Result<(), sqlx::Error>;
	async fn delete_user(&self, user_id: Uuid) -> Result<(), sqlx::Error>;
}

/// Sqlx implementation of the `UserRepository` trait.
pub struct SqlxUserRepository {
	pool: sqlx::SqlitePool,
}

impl SqlxUserRepository {
	pub fn new(pool: sqlx::SqlitePool) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, sqlx::Error> {
		let user = sqlx::query_as!(
			User,
			r#"SELECT id as "id: uuid::Uuid", fullname, username, password_hash FROM users WHERE id = ?"#,
			user_id
		)
		.fetch_one(&self.pool)
		.await;

		user
	}

	async fn get_user_by_username(&self, username: &str) -> Result<User, sqlx::Error> {
		let user = sqlx::query_as!(
			User,
			r#"SELECT id as "id: uuid::Uuid", fullname, username, password_hash FROM users WHERE username = ?"#,
			username
		)
		.fetch_one(&self.pool)
		.await;
		user
	}

	async fn update_user_username(&self, user_id: Uuid, username: &str) -> Result<(), sqlx::Error> {
		sqlx::query!(
			r#"UPDATE users SET username = ? WHERE id = ?"#,
			username,
			user_id
		)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

	async fn update_user_fullname(&self, user_id: Uuid, fullname: &str) -> Result<(), sqlx::Error> {
		sqlx::query!(
			r#"UPDATE users SET fullname = ? WHERE id = ?"#,
			fullname,
			user_id
		)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

	async fn update_user_password_hash(
		&self,
		user_id: Uuid,
		password_hash: &str,
	) -> Result<(), sqlx::Error> {
		sqlx::query!(
			r#"UPDATE users SET password_hash = ? WHERE id = ?"#,
			password_hash,
			user_id
		)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

	async fn create_user(
		&self,
		user_id: Uuid,
		fullname: &str,
		username: &str,
		password_hash: &str,
	) -> Result<(), sqlx::Error> {
		sqlx::query!(
			r#"INSERT INTO users (id, fullname, username, password_hash) VALUES (?, ?, ?, ?)"#,
			user_id,
			fullname,
			username,
			password_hash
		)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

	async fn delete_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
		sqlx::query!(r#"DELETE FROM users WHERE id = ?"#, user_id)
			.execute(&self.pool)
			.await?;

		Ok(())
	}
}
