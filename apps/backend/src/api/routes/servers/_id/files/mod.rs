use crate::AppState;
use axum::response::IntoResponse;
use axum::{routing, Router};
use reqwest::StatusCode;
use std::sync::Arc;

mod _path;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get).post(post))
		.nest("/{path}", _path::create_router())
}

async fn post() -> impl IntoResponse {
	// TODO: Create a new file
	StatusCode::CREATED.into_response()
}

async fn get() -> impl IntoResponse {
	// TODO: Get file listings
	StatusCode::OK.into_response()
}
