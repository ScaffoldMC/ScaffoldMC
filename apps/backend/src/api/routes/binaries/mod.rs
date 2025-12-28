use crate::AppState;
use axum::{extract::State, response::IntoResponse, routing, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

/// Get the list of installed binaries.
pub async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	return (
		axum::http::StatusCode::OK,
		Json(state.binary_service.get_installed().await),
	);
}
