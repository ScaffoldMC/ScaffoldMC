use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum::{routing, Router};
use log::error;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::config::{AUTH_COOKIE_NAME, REFRESH_COOKIE_NAME};
use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/login", routing::post(post))
}

async fn post(cookies: Cookies, State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let refresh_token = cookies
		.get(REFRESH_COOKIE_NAME)
		.map(|cookie| cookie.value().to_string());

	if let Some(ref_token) = refresh_token {
		if let Err(err) = state.auth_service.delete_refresh_token(&ref_token).await {
			error!("Failed to delete refresh token: {}", err);
			return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
		}
	}

	cookies.remove(Cookie::build(REFRESH_COOKIE_NAME).path("/").build());
	cookies.remove(Cookie::build(AUTH_COOKIE_NAME).path("/").build());

	StatusCode::OK.into_response()
}
