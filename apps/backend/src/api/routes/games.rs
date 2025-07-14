use crate::AppState;
use axum::{extract::State, response::IntoResponse, routing, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

pub async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let games = state.binary_service.get_games().await.map_err(|e| {
		(
			axum::http::StatusCode::INTERNAL_SERVER_ERROR,
			format!("Internal server error: {}", e),
		)
			.into_response()
	});

	Json(games.unwrap()).into_response()
}
