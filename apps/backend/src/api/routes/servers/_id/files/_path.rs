use std::sync::Arc;

use axum::{response::IntoResponse, routing, Router};
use reqwest::StatusCode;

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::patch(patch).delete(delete))
}

async fn delete() -> impl IntoResponse {
	// TODO: Delete a file
	StatusCode::NO_CONTENT.into_response()
}

async fn patch() -> impl IntoResponse {
	// TODO: Update a file contents
	StatusCode::OK.into_response()
}
