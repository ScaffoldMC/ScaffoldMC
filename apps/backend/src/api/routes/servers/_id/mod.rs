use std::sync::Arc;

use axum::{response::IntoResponse, routing, Router};
use reqwest::StatusCode;

use crate::AppState;

mod console;
mod files;
mod status;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::patch(patch))
		.route("/", routing::delete(delete))
		.nest("/status", status::create_router())
		.nest("/files", files::create_router())
		.nest("/console", console::create_router())
}

async fn get() -> impl IntoResponse {
	// TODO: Return server information
	StatusCode::OK.into_response()
}

async fn patch() -> impl IntoResponse {
	// TODO: Update server information
	StatusCode::OK.into_response()
}

async fn delete() -> impl IntoResponse {
	// TODO: Move server to trash
	StatusCode::OK.into_response()
}
