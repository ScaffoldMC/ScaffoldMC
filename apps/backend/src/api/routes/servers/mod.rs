use crate::AppState;
use axum::response::IntoResponse;
use axum::{routing, Router};
use reqwest::StatusCode;
use std::sync::Arc;

mod _id;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
		.nest("/{id}", _id::create_router())
}

async fn get() -> impl IntoResponse {
	// TODO: Return server listings
	StatusCode::OK.into_response()
}

async fn post() -> impl IntoResponse {
	// TODO: Create a new server
	StatusCode::CREATED.into_response()
}
