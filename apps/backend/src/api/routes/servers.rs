use crate::api::types::server::PartialServer;
use crate::AppState;
use axum::{extract::State, response::IntoResponse};
use axum::{routing::get, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(get_servers))
}

pub async fn get_servers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	todo!("Call server service")
}
