use crate::{config::AUTH_COOKIE_NAME, services::auth::AuthServiceError, AppState};
use axum::{
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::Response,
};
use std::sync::Arc;
use tower_cookies::Cookies;

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

	let user = match state.auth_service.get_user_from_token(&token, false).await {
		Ok(user) => user,
		Err(err) => match err {
			AuthServiceError::Unauthorized | AuthServiceError::InvalidCredentials => {
				return Err(StatusCode::UNAUTHORIZED)
			}
			_ => {
				tracing::error!("Authentication error: {}", err);
				return Err(StatusCode::INTERNAL_SERVER_ERROR);
			}
		},
	};

	req.extensions_mut().insert(user);
	Ok(next.run(req).await)
}

pub async fn require_sudo(
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

	let user = match state.auth_service.get_user_from_token(&token, true).await {
		Ok(user) => user,
		Err(err) => match err {
			AuthServiceError::Unauthorized | AuthServiceError::InvalidCredentials => {
				return Err(StatusCode::UNAUTHORIZED)
			}
			AuthServiceError::Forbidden => return Err(StatusCode::FORBIDDEN),
			AuthServiceError::ServerError(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
		},
	};

	req.extensions_mut().insert(user);
	Ok(next.run(req).await)
}
