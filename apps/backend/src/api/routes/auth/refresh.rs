use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum::{routing, Router};
use log::error;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::config::{AUTH_COOKIE_NAME, REFRESH_COOKIE_NAME};
use crate::services::auth::AuthServiceError;
use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::post(post))
}

// FIXME: Unused refresh tokens need to be cleared from the db occasionally (perhaps on program startup?)

async fn post(cookies: Cookies, State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let ref_token = cookies
		.get(REFRESH_COOKIE_NAME)
		.map(|cookie| cookie.value().to_string());

	if ref_token.is_none() {
		return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
	}

	let (auth_token, new_ref_token) = match state
		.auth_service
		.refresh_tokens(ref_token.as_ref().unwrap())
		.await
	{
		Ok(tokens) => tokens,
		Err(err) => match err {
			AuthServiceError::Unauthorized => {
				return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
			}
			AuthServiceError::InvalidCredentials => {
				return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
			}
			AuthServiceError::ServerError(err) => {
				error!("Failed to refresh tokens: {}", err);
				return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
			}
		},
	};

	let new_ref_cookie = Cookie::build((REFRESH_COOKIE_NAME, new_ref_token))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(new_ref_cookie);

	let new_auth_cookie = Cookie::build((AUTH_COOKIE_NAME, auth_token.clone()))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(new_auth_cookie);

	StatusCode::OK.into_response()
}
