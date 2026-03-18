use std::sync::Arc;

use axum::{
	extract::{Path, State},
	response::IntoResponse,
	routing, Json, Router,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{api::types::server::ServerCommandRequest, services::server::ServerError, AppState};

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
}

async fn get() -> impl IntoResponse {
	// TODO: Return console output stream
	StatusCode::OK.into_response()
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
		Err(err) => match err {
			ServerError::NoSuchServer(_) => {
				tracing::error!("Server not found: {}", id);
				return StatusCode::NOT_FOUND.into_response();
			}
			_ => {
				tracing::error!("Error sending command to server {}: {}", id, err);
				StatusCode::INTERNAL_SERVER_ERROR.into_response()
			}
		},
	}
}
