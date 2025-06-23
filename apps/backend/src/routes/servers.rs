use crate::AppState;
use axum::{extract::State, response::IntoResponse};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(get_servers))
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
struct PartialServer {
	id: String,
	name: String,
}

pub async fn get_servers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let servers = state
		.servers
		.iter()
		.map(|server| PartialServer {
			id: server.id.clone(),
			name: server.name.clone(),
		})
		.collect::<Vec<PartialServer>>();

	Json(servers).into_response()
}
