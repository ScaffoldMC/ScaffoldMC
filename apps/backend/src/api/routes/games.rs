use crate::AppState;
use axum::{extract::State, response::IntoResponse, routing, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

pub async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	todo!("Return list of games")
}
