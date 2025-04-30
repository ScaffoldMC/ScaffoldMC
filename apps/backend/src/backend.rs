use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use serde::Deserialize;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::task;

#[derive(Clone)]
pub struct Backend {
	db: SqlitePool,
}

#[derive(Error, Debug)]
pub enum AuthError {
	#[error(transparent)]
	Sqlx(#[from] sqlx::Error),

	#[error(transparent)]
	TaskJoin(#[from] tokio::task::JoinError),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[async_trait]
impl AuthnBackend for Backend {
	type User = crate::user::User;
	type Credentials = Credentials;
	type Error = AuthError;

	async fn authenticate(
		&self,
		creds: Self::Credentials,
	) -> Result<Option<Self::User>, Self::Error> {
		let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
			.bind(creds.username)
			.fetch_optional(&self.db)
			.await?;

		// Verifying the password is blocking and potentially slow, so we'll do so via
		// `spawn_blocking`.
		task::spawn_blocking(|| {
			// We're using password-based authentication--this works by comparing our form
			// input with an argon2 password hash.
			Ok(user.filter(|user| verify_password(creds.password, &user.password_hash()).is_ok()))
		})
		.await?
	}

	async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
		let user = sqlx::query_as("select * from users where id = ?")
			.bind(user_id)
			.fetch_optional(&self.db)
			.await?;

		Ok(user)
	}
}

pub type AuthSession = axum_login::AuthSession<Backend>;
