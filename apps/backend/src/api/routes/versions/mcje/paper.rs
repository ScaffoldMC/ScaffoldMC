use crate::{
	core::game::{
		java::{MinecraftJava, MinecraftJavaLoader},
		Game,
	},
	AppState,
};
use axum::{
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
	Json, Router,
};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", axum::routing::get(get))
		.route("/{game_version}", axum::routing::get(get_loader))
		.route(
			"/{game_version}/{loader_version}",
			axum::routing::get(get_game),
		)
}

pub async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let versions_res = state.binary_service.paper.list_game_versions().await;

	if let Err(err) = versions_res {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Internal server error: {}", err),
		)
			.into_response()
	} else {
		(StatusCode::OK, Json(versions_res.unwrap())).into_response()
	}
}

pub async fn get_loader(
	State(state): State<Arc<AppState>>,
	Path(game_version): Path<String>,
) -> impl IntoResponse {
	let versions_res = state
		.binary_service
		.paper
		.list_loader_versions(&game_version)
		.await;

	if let Err(err) = versions_res {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Internal server error: {}", err),
		)
			.into_response()
	} else {
		(StatusCode::OK, Json(versions_res.unwrap())).into_response()
	}
}

pub async fn get_game(
	State(state): State<Arc<AppState>>,
	Path((game_version, loader_version)): Path<(String, u16)>,
) -> impl IntoResponse {
	let versions_res = state
		.binary_service
		.paper
		.list_loader_versions(&game_version)
		.await;

	if let Err(err) = versions_res {
		return (
			StatusCode::NOT_FOUND,
			format!("Error verifying game version {}: {}", game_version, err),
		)
			.into_response();
	}

	let versions = versions_res.unwrap();

	if !versions.contains(&loader_version) {
		return (
			StatusCode::NOT_FOUND,
			format!(
				"Loader version {} for game version {} not found",
				loader_version, game_version
			),
		)
			.into_response();
	}

	let game = Game::MinecraftJava(MinecraftJava {
		version: game_version,
		loader: MinecraftJavaLoader::Paper {
			build: loader_version,
		},
	});

	(StatusCode::OK, Json(game)).into_response()
}
