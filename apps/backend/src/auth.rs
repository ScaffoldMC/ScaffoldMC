use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};
use base64::engine::general_purpose;
use base64::Engine;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use log::error;
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

use crate::db::Database;

pub static REFRESH_TOKEN_LENGTH: Duration = Duration::hours(6);
pub static AUTH_TOKEN_LENGTH: Duration = Duration::minutes(5);

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenClaims {
	pub iat: i64,
	pub exp: i64,
	pub sub: String,
}

pub fn create_auth_token(user_id: String) -> String {
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
		&jsonwebtoken::Header::default(),
		&auth_jwt_claims,
		&EncodingKey::from_secret(b"hunter2"), // TODO: Make randomized secret
	)
	.expect("Failed to create auth token")
}

pub fn create_refresh_token() -> String {
	let mut bytes = [0u8; 32];
	rand::rng().fill(&mut bytes);
	general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

async fn require_auth(
	State(db): State<Database>,
	mut req: Request,
	next: Next,
) -> Result<Response, StatusCode> {
	let token = match req.headers().get("Authorization") {
		Some(token) => token,
		None => return Err(StatusCode::UNAUTHORIZED),
	};

	let token_str = token.to_str().unwrap_or("");
	let tk_data = match jsonwebtoken::decode::<AuthTokenClaims>(
		token_str,
		&DecodingKey::from_secret(b"hunter2"),
		&Validation::new(Algorithm::HS256),
	) {
		Ok(data) => data,
		Err(_) => return Err(StatusCode::UNAUTHORIZED),
	};

	let user_uuid = match Uuid::parse_str(&tk_data.claims.sub) {
		Ok(uuid) => uuid,
		Err(err) => {
			error!("Unable to parse UUID: {}", err);
			return Err(StatusCode::UNAUTHORIZED);
		}
	};

	let user = match db.get_user_by_id(user_uuid).await {
		Ok(user) => user,
		Err(_) => return Err(StatusCode::UNAUTHORIZED),
	};

	req.extensions_mut().insert(user);
	Ok(next.run(req).await)
}
