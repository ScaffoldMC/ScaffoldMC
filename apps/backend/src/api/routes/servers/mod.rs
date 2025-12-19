use crate::api::types::server::CreateServerRequest;
use crate::core::game::Game;
use crate::core::version::vanilla::VanillaVersionInfo;
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

async fn get() -> impl IntoResponse {
	// TODO: Return server listings
	StatusCode::OK.into_response()
}

#[axum::debug_handler]
async fn post(
	State(state): State<Arc<AppState>>,
	Json(req): Json<CreateServerRequest>,
) -> impl IntoResponse {
	let create_res = state
		.server_service
		.create(
			&req.name,
			Game::MinecraftJava {
				version: VanillaVersionInfo::new("1.21.11".into()),
			},
		)
		.await;

	let server_id = match create_res {
		Ok(id) => id,
		Err(err) => {
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				format!("Failed to create server: {}", err),
			)
				.into_response();
		}
	};

	StatusCode::CREATED.into_response()
}
