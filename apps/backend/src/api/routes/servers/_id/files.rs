use crate::AppState;
use axum::response::IntoResponse;
use axum::{routing, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route(
		"/{*path}",
		routing::get(get).post(post).delete(delete).put(put),
	)
}

// TODO: Prevent path traversal attacks in all routes!!!

async fn post() -> impl IntoResponse {
	// TODO: Create a new file
	StatusCode::CREATED.into_response()
}

async fn get() -> impl IntoResponse {
	// TODO: Get file listings
	StatusCode::OK.into_response()
}

async fn delete() -> impl IntoResponse {
	// TODO: Delete a file
	StatusCode::NO_CONTENT.into_response()
}

async fn put() -> impl IntoResponse {
	// TODO: Update a file contents
	StatusCode::OK.into_response()
}
