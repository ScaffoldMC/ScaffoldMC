use std::sync::Arc;

use axum::{response::IntoResponse, routing, Router};
use reqwest::StatusCode;

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

async fn get() -> impl IntoResponse {
	// TODO: Return stream of status information, like player count, uptime, resource usage, etc.
	StatusCode::OK.into_response()
}
