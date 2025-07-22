use std::sync::Arc;

use axum::{response::IntoResponse, routing, Router};
use reqwest::StatusCode;

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
}

async fn get() -> impl IntoResponse {
	// TODO: Return console output stream
	StatusCode::OK.into_response()
}

async fn post() -> impl IntoResponse {
	// TODO: Send command to console
	StatusCode::OK.into_response()
}
