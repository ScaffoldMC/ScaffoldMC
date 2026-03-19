use std::sync::Arc;

use axum::{
	extract::{Path, Query, State},
	response::IntoResponse,
	routing, Json, Router,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
	api::types::server::{ConsoleQueryParams, ServerCommandRequest},
	services::server::ServerError,
	AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
}

async fn get(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Query(query): Query<ConsoleQueryParams>,
) -> impl IntoResponse {
	let console_lines = match state
		.server_service
		.get_console_snapshot(id, query.since)
		.await
	{
		Ok(lines) => lines,
		Err(err) => {
			tracing::error!("Error getting console lines for server {}: {}", id, err);
			return StatusCode::INTERNAL_SERVER_ERROR.into_response();
		}
	};

	(StatusCode::OK, Json(console_lines)).into_response()
}

async fn post(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Json(request): Json<ServerCommandRequest>,
) -> impl IntoResponse {
	match state
		.server_service
		.send_command(id, &request.command)
		.await
	{
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			if let ServerError::NoSuchServer(_) = err {
				tracing::error!("Server not found: {}", id);
				StatusCode::NOT_FOUND.into_response()
			} else {
				tracing::error!("Error sending command to server {}: {}", id, err);
				StatusCode::INTERNAL_SERVER_ERROR.into_response()
			}
		}
	}
}
