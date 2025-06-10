use crate::AppState;
use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};
use base64::engine::general_purpose;
use base64::Engine;
use jsonwebtoken::{Algorithm, Validation};
use log::error;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Duration;
use tower_cookies::Cookies;
use uuid::Uuid;

pub static REFRESH_COOKIE_NAME: &str = "ref_token";
pub static AUTH_COOKIE_NAME: &str = "auth_token";

pub static REFRESH_TOKEN_LENGTH: Duration = Duration::hours(6);
pub static AUTH_TOKEN_LENGTH: Duration = Duration::minutes(5);

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

pub async fn require_auth(
	cookies: Cookies,
	State(state): State<Arc<AppState>>,
	mut req: Request,
	next: Next,
) -> Result<Response, StatusCode> {
	let token = match req
		.headers()
		.get("Authorization")
		.and_then(|header| {
			let header_str = header.to_str().ok()?;
			let parts = header_str.split(' ').collect::<Vec<&str>>();
			if parts.len() == 2 && parts[0] == "Bearer" {
				Some(parts[1].to_string())
			} else {
				None
			}
		})
		.or_else(|| {
			cookies
				.get(AUTH_COOKIE_NAME)
				.map(|cookie| cookie.value().to_string())
		}) {
		Some(token) => token,
		None => return Err(StatusCode::UNAUTHORIZED),
	};

	let token_data = match jsonwebtoken::decode::<AuthTokenClaims>(
		&token,
		&state.secrets.jwt_dec,
		&Validation::new(Algorithm::RS256),
	) {
		Ok(data) => data,
		Err(_) => return Err(StatusCode::UNAUTHORIZED),
	};

	let user_uuid = match Uuid::parse_str(&token_data.claims.sub) {
		Ok(uuid) => uuid,
		Err(err) => {
			error!("Unable to parse UUID: {}", err);
			return Err(StatusCode::UNAUTHORIZED);
		}
	};

	let user = match state.db.get_user_by_id(user_uuid).await {
		Ok(user) => user,
		Err(_) => return Err(StatusCode::UNAUTHORIZED),
	};

	req.extensions_mut().insert(user);
	Ok(next.run(req).await)
}
