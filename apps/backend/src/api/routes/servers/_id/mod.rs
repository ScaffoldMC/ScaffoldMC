use std::sync::Arc;

use axum::{
	extract::{Path, State},
	response::IntoResponse,
	routing, Json, Router,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{services::server::ServerError, AppState};

mod console;
mod files;
mod status;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::patch(patch))
		.route("/", routing::delete(delete))
		.nest("/status", status::create_router())
		.nest("/files", files::create_router())
		.nest("/console", console::create_router())
}

async fn get(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
	let server_info = state.server_service.get_server_info(id).await;

	if let Err(err) = server_info {
		match err {
			ServerError::NoSuchServer(_) => {
				return (StatusCode::NOT_FOUND, err.to_string()).into_response()
			}
			_ => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
		}
	}

	(StatusCode::OK, Json(server_info.unwrap())).into_response()
}

async fn patch() -> impl IntoResponse {
	// TODO: Update server information
	StatusCode::OK.into_response()
}

async fn delete() -> impl IntoResponse {
	// TODO: Move server to trash
	StatusCode::OK.into_response()
}
