use crate::{
	api::middleware::server::require_server,
	core::{files::server_config::PartialServerConfig, server::Server},
	AppState,
};
use axum::{
	extract::{Path, State},
	middleware,
	response::IntoResponse,
	routing, Extension, Json, Router,
};
use reqwest::StatusCode;
use std::sync::Arc;
use uuid::Uuid;

mod console;
mod files;
mod status;

pub fn create_router(state: &Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::delete(delete))
		.route("/start", routing::post(start_post))
		.route("/stop", routing::post(stop_post))
		.route("/kill", routing::post(kill_post))
		.route("/config", routing::patch(config_patch))
		.nest("/status", status::create_router())
		.nest("/files", files::create_router())
		.nest("/console", console::create_router())
		.route_layer(middleware::from_fn_with_state(
			state.clone(),
			require_server,
		))
}

async fn config_patch(
	Extension(server): Extension<Arc<Server>>,
	Json(config): Json<PartialServerConfig>,
) -> impl IntoResponse {
	match server.update_config(config).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error updating server config: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn start_post(
	State(state): State<Arc<AppState>>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	if let Err(err) = server.start(state.binary_service.clone()).await {
		tracing::error!("Error starting server: {}", err);
		StatusCode::INTERNAL_SERVER_ERROR.into_response()
	} else {
		StatusCode::OK.into_response()
	}
}

async fn stop_post(Extension(server): Extension<Arc<Server>>) -> impl IntoResponse {
	match server.stop().await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error stopping server: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn kill_post(Extension(server): Extension<Arc<Server>>) -> impl IntoResponse {
	match server.kill().await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error killing server: {}", err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}

async fn get(Extension(server): Extension<Arc<Server>>) -> impl IntoResponse {
	let server_info = server.get_server_info().await;

	(StatusCode::OK, Json(server_info)).into_response()
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
