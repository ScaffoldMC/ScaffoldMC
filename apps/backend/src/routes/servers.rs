use crate::api_types::server::PartialServer;
use crate::AppState;
use axum::{extract::State, response::IntoResponse};
use axum::{routing::get, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(get_servers))
}

pub async fn get_servers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let servers = state
		.servers
		.read()
		.await
		.iter()
		.map(|(&id, instance)| PartialServer {
			id: id.clone(),
			name: instance.config.name.clone(),
		})
		.collect::<Vec<PartialServer>>();

	Json(servers).into_response()
}
