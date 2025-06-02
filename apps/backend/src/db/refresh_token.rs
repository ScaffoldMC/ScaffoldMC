use super::Database;
use crate::auth::REFRESH_TOKEN_LENGTH;
use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, types::Uuid, FromRow};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
	pub id: String,
	pub user_id: Uuid,
	pub created_at: OffsetDateTime,
}

impl Database {
	pub async fn add_refresh_token(
		&self,
		token_id: &str,
		user_id: Uuid,
	) -> Result<(), sqlx::Error> {
		// Then insert the new token
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
			return Err(result.err().unwrap());
		} else {
			Ok(())
		}
	}

	pub async fn delete_refresh_token(&self, token_id: &str) -> Result<(), sqlx::Error> {
		let result = sqlx::query!(r#"DELETE FROM refresh_tokens WHERE id = ?"#, token_id)
			.execute(&self.pool)
			.await;

		if result.is_err() {
			return Err(result.err().unwrap());
		} else {
			Ok(())
		}
	}

	pub async fn get_refresh_token(
		&self,
		token_id: &str,
	) -> Result<Option<RefreshToken>, sqlx::Error> {
		sqlx::query_as!(
			RefreshToken,
			r#"SELECT id, user_id as "user_id: uuid::Uuid", created_at as "created_at: OffsetDateTime" FROM refresh_tokens WHERE id = ?"#,
			token_id
		)
		.fetch_optional(&self.pool)
		.await
	}

	pub async fn purge_refresh_tokens(&self) -> Result<(), sqlx::Error> {
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
