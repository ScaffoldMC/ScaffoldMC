use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

pub fn create_router() -> Router {
	Router::new().route("/login", post(login))
}

pub async fn login(Json(creds): Json<Credentials>) -> impl IntoResponse {
	// TODO: Login logic.
	// Note: Use spawn_blocking for password verification.

	StatusCode::OK.into_response()
}
