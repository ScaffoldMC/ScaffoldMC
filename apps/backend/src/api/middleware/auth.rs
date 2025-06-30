use crate::{auth::AuthTokenClaims, config::AUTH_COOKIE_NAME, AppState};
use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};
use jsonwebtoken::{Algorithm, Validation};
use log::error;
use std::sync::Arc;
use tower_cookies::Cookies;
use uuid::Uuid;

pub async fn require_auth(
	cookies: Cookies,
	State(state): State<Arc<AppState>>,
	mut req: Request,
	next: Next,
) -> Result<Response, StatusCode> {
	let token = cookies
		.get(AUTH_COOKIE_NAME)
		.map(|cookie| cookie.value().to_string());

	if token.is_none() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let token = token.unwrap();

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
