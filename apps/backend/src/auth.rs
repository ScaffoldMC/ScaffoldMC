use crate::{config::AUTH_TOKEN_LENGTH, AppState};
use base64::engine::general_purpose;
use base64::Engine;
use jsonwebtoken::Algorithm;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenClaims {
	pub iat: i64,
	pub exp: i64,
	pub sub: String,
}

pub fn create_auth_token(state: &Arc<AppState>, user_id: String) -> String {
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
	};

	jsonwebtoken::encode(
		&jsonwebtoken::Header::new(Algorithm::RS256),
		&auth_jwt_claims,
		&state.secrets.jwt_enc,
	)
	.expect("Failed to create auth token")
}

pub fn create_refresh_token() -> String {
	let mut bytes = [0u8; 32];
	rand::rng().fill(&mut bytes);
	general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}
