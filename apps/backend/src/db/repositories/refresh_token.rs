use crate::{config::REFRESH_TOKEN_LENGTH, db::models::refresh_token::RefreshToken};
use async_trait::async_trait;
use sqlx::{types::time::OffsetDateTime, types::Uuid};

/// Repository structure for managing refresh tokens in the database.
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
	async fn add_refresh_token(&self, token_id: &str, user_id: Uuid) -> Result<(), sqlx::Error>;
	async fn delete_refresh_token(&self, token_id: &str) -> Result<(), sqlx::Error>;
	async fn get_refresh_token(&self, token_id: &str) -> Result<Option<RefreshToken>, sqlx::Error>;
	async fn purge_refresh_tokens(&self) -> Result<(), sqlx::Error>;
}

/// Sqlx implementation of the `RefreshTokenRepository` trait.
pub struct SqlxRefreshTokenRepository {
	pool: sqlx::SqlitePool,
}

impl SqlxRefreshTokenRepository {
	pub fn new(pool: sqlx::SqlitePool) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl RefreshTokenRepository for SqlxRefreshTokenRepository {
	async fn add_refresh_token(&self, token_id: &str, user_id: Uuid) -> Result<(), sqlx::Error> {
		let now = OffsetDateTime::now_utc();

		let result = sqlx::query!(
			r#"INSERT INTO refresh_tokens (id, user_id, created_at) VALUES (?, ?, ?)"#,
			token_id,
			user_id,
			now
		)
		.execute(&self.pool)
		.await;

		if result.is_err() {
			Err(result.err().unwrap())
		} else {
			Ok(())
		}
	}

	async fn delete_refresh_token(&self, token_id: &str) -> Result<(), sqlx::Error> {
		let result = sqlx::query!(r#"DELETE FROM refresh_tokens WHERE id = ?"#, token_id)
			.execute(&self.pool)
			.await;

		if result.is_err() {
			Err(result.err().unwrap())
		} else {
			Ok(())
		}
	}

	async fn get_refresh_token(&self, token_id: &str) -> Result<Option<RefreshToken>, sqlx::Error> {
		sqlx::query_as!(
			RefreshToken,
			r#"SELECT id, user_id as "user_id: uuid::Uuid", created_at as "created_at: OffsetDateTime" FROM refresh_tokens WHERE id = ?"#,
			token_id
		)
		.fetch_optional(&self.pool)
		.await
	}

	async fn purge_refresh_tokens(&self) -> Result<(), sqlx::Error> {
		let earliest_timestamp = OffsetDateTime::now_utc() - REFRESH_TOKEN_LENGTH;

		sqlx::query!(
			r#"DELETE FROM refresh_tokens WHERE created_at < ?"#,
			earliest_timestamp
		)
		.execute(&self.pool)
		.await?;

		Ok(())
	}
}
