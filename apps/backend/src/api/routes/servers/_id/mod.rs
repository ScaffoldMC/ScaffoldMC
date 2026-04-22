use std::sync::Arc;

use axum::{
	extract::{Path, State},
	response::IntoResponse,
	routing, Json, Router,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
	core::files::server_config::PartialServerConfig, services::server::ServerError, AppState,
};

mod console;
mod files;
mod status;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::patch(patch))
		.route("/", routing::delete(delete))
		.route("/start", routing::post(start_post))
		.route("/stop", routing::post(stop_post))
		.route("/kill", routing::post(kill_post))
		.route("/config", routing::patch(config_patch))
		.nest("/status", status::create_router())
		.nest("/files", files::create_router())
		.nest("/console", console::create_router())
}

async fn config_patch(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Json(config): Json<PartialServerConfig>,
) -> impl IntoResponse {
	match state.server_service.update_config(id, config).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error updating server config: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn start_post(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
	match state.server_service.start(id).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error starting server: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn stop_post(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
	match state.server_service.stop(id).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error stopping server: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn kill_post(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
	match state.server_service.kill(id).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error killing server: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn get(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
	let server_info = state.server_service.get_server_info(id).await;

	if let Err(err) = server_info {
		if let ServerError::NoSuchServer(_) = err {
			return (StatusCode::NOT_FOUND, err.to_string()).into_response();
		}
		tracing::error!("Error retrieving server info: {}", err);
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}

	(StatusCode::OK, Json(server_info.unwrap())).into_response()
}

async fn patch() -> impl IntoResponse {
	// TODO: Update server information
	StatusCode::OK.into_response()
}

async fn delete(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
	match state.server_service.delete(id).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error deleting server: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}
