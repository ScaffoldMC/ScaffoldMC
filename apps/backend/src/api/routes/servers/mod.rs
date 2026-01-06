use crate::api::types::server::CreateServerRequest;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing, Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;

mod _id;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
		.nest("/{id}", _id::create_router())
}

async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let servers = state.server_service.list_servers().await;
	(StatusCode::OK, Json(servers)).into_response()
}

async fn post(
	State(state): State<Arc<AppState>>,
	Json(req): Json<CreateServerRequest>,
) -> impl IntoResponse {
	let result = state.server_service.create(&req.name, req.game).await;

	if let Err(err) = result {
		return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
	}

	StatusCode::CREATED.into_response()
}
