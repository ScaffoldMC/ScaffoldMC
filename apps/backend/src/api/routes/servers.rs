use crate::AppState;
use axum::response::IntoResponse;
use axum::{routing, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

pub async fn get() -> impl IntoResponse {
	StatusCode::OK.into_response()
}
