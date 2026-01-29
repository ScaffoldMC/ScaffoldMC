use crate::{
	config::{AUTH_TOKEN_LENGTH, REFRESH_TOKEN_LENGTH},
	core::secrets::Secrets,
	db::{user::User, Database},
	services::Service,
};
use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{Algorithm, Validation};
use password_auth::{verify_password, VerifyError};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror;
use tokio::task::spawn_blocking;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AuthServiceError {
	#[error("Invalid credentials")]
	InvalidCredentials,
	#[error("Unauthorized access")]
	Unauthorized,
	#[error("Forbidden access")]
	Forbidden,
	#[error("Internal server error: {0}")]
	ServerError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenClaims {
	pub iat: i64,
	pub exp: i64,
	pub sub: String,
	pub sudo: bool,
}

pub struct AuthService {
	db: Arc<Database>,
	secrets: Secrets,
}

impl Service for AuthService {}

impl AuthService {
	pub fn new(db: Arc<Database>, secrets: Secrets) -> Self {
		Self { db, secrets }
	}

	fn create_auth_token(&self, user_id: String, sudo: bool) -> String {
		let time_now = time::UtcDateTime::now();
		let issued_at_secs = time_now.unix_timestamp();
		let expiration_secs = time_now
			.checked_add(AUTH_TOKEN_LENGTH)
			.expect("Failed to add duration")
			.unix_timestamp();

		let auth_jwt_claims = AuthTokenClaims {
			iat: issued_at_secs,
			exp: expiration_secs,
			sub: user_id,
			sudo,
		};

		jsonwebtoken::encode(
			&jsonwebtoken::Header::new(Algorithm::RS256),
			&auth_jwt_claims,
			&self.secrets.jwt_enc,
		)
		.expect("Failed to create auth token")
	}

	fn create_refresh_token() -> String {
		let mut bytes = [0u8; 32];
		rand::rng().fill(&mut bytes);
		general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
	}

	pub async fn verify_password(
		&self,
		user: &User,
		password: &str,
	) -> Result<(), AuthServiceError> {
		let password_owned = password.to_owned();
		let password_hash_owned = user.password_hash.clone();
		let verify_result =
			spawn_blocking(move || verify_password(&password_owned, &password_hash_owned))
				.await
				.map_err(|e| AuthServiceError::ServerError(e.to_string()))?;

		if let Err(err) = verify_result {
			return match err {
				VerifyError::PasswordInvalid => Err(AuthServiceError::InvalidCredentials),
				VerifyError::Parse(e) => Err(AuthServiceError::ServerError(e.to_string())),
			};
		}

		Ok(())
	}

	pub async fn authenticate_user(
		&self,
		username: &str,
		password: &str,
	) -> Result<(String, String), AuthServiceError> {
		let user = self.db.get_user_by_username(username).await;

		if let Err(_) = user {
			return Err(AuthServiceError::InvalidCredentials);
		}

		let user = user.unwrap();

		self.verify_password(&user, password).await?;

		let auth_token = self.create_auth_token(user.id.to_string(), false);
		let ref_token = Self::create_refresh_token();

		self.db
			.add_refresh_token(&ref_token, user.id)
			.await
			.map_err(|e| AuthServiceError::ServerError(e.to_string()))?;

		Ok((auth_token, ref_token))
	}

	pub async fn sudo_user(&self, user: User, password: &str) -> Result<String, AuthServiceError> {
		self.verify_password(&user, password).await?;

		let sudo_token = self.create_auth_token(user.id.to_string(), true);

		Ok(sudo_token)
	}

	pub async fn refresh_tokens(
		&self,
		ref_token: &str,
	) -> Result<(String, String), AuthServiceError> {
		let db_entry = self.db.get_refresh_token(ref_token).await;

		if let Err(err) = db_entry {
			return Err(AuthServiceError::ServerError(err.to_string()));
		}

		let db_entry = db_entry.unwrap();

		if db_entry.is_none() {
			return Err(AuthServiceError::Unauthorized);
		}

		let db_entry = db_entry.unwrap();

		let current_time = time::OffsetDateTime::now_utc();
		let token_age = current_time - db_entry.created_at;

		if token_age > REFRESH_TOKEN_LENGTH {
			if let Err(err) = self.db.delete_refresh_token(&ref_token).await {
				return Err(AuthServiceError::ServerError(err.to_string()));
			}

			return Err(AuthServiceError::Unauthorized);
		}

		if let Err(err) = self.db.delete_refresh_token(&ref_token).await {
			return Err(AuthServiceError::ServerError(err.to_string()));
		}

		let auth_token = self.create_auth_token(db_entry.user_id.to_string(), false);
		let new_ref_token = Self::create_refresh_token();

		if let Err(err) = self
			.db
			.add_refresh_token(&new_ref_token, db_entry.user_id)
			.await
		{
			return Err(AuthServiceError::ServerError(err.to_string()));
		}

		Ok((auth_token, new_ref_token))
	}

	pub async fn delete_refresh_token(&self, ref_token: &str) -> Result<(), AuthServiceError> {
		let result = self.db.delete_refresh_token(ref_token);

		if let Err(err) = result.await {
			return Err(AuthServiceError::ServerError(err.to_string()));
		}

		Ok(())
	}

	pub async fn get_user_from_token(
		&self,
		token: &str,
		sudo: bool,
	) -> Result<User, AuthServiceError> {
		let token_data = match jsonwebtoken::decode::<AuthTokenClaims>(
			&token,
			&self.secrets.jwt_dec,
			&Validation::new(Algorithm::RS256),
		) {
			Ok(data) => data,
			Err(_) => return Err(AuthServiceError::Unauthorized),
		};

		if sudo && !token_data.claims.sudo {
			return Err(AuthServiceError::Forbidden);
		}

		let user_uuid = match Uuid::parse_str(&token_data.claims.sub) {
			Ok(uuid) => uuid,
			Err(err) => {
				tracing::error!("Unable to parse UUID: {}", err);
				return Err(AuthServiceError::Unauthorized);
			}
		};

		let user = match self.db.get_user_by_id(user_uuid).await {
			Ok(user) => user,
			Err(_) => return Err(AuthServiceError::Unauthorized),
		};

		Ok(user)
	}

	pub async fn token_is_sudo(&self, token: &str) -> Result<bool, AuthServiceError> {
		let token_data = match jsonwebtoken::decode::<AuthTokenClaims>(
			&token,
			&self.secrets.jwt_dec,
			&Validation::new(Algorithm::RS256),
		) {
			Ok(data) => data,
			Err(_) => return Err(AuthServiceError::Unauthorized),
		};

		Ok(token_data.claims.sudo)
	}
}
