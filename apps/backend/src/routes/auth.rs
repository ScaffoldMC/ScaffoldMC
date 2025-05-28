use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use super::RouterWithState;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

pub fn create_router() -> RouterWithState {
	Router::new()
		.route("/login", post(login))
		.route("/logout", post(logout))
		.route("/refresh", post(refresh))
}

pub async fn login(Json(creds): Json<Credentials>) -> impl IntoResponse {
	// TODO: Login logic.
	// Note: Use spawn_blocking for password verification.

	StatusCode::OK.into_response()
}

pub async fn refresh() -> impl IntoResponse {
	// TODO: Refresh token logic. Allow with header or cookie.

	StatusCode::OK.into_response()
}

pub async fn logout() -> impl IntoResponse {
	// TODO: Logout logic.
	StatusCode::OK.into_response()
}
