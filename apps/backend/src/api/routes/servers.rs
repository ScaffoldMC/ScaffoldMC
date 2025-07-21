use crate::AppState;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(get_servers))
}

pub async fn get_servers() -> impl IntoResponse {
	StatusCode::OK.into_response()
}
