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
		.route("/vanilla", axum::routing::get(get_vanilla))
		.route("/vanilla/{version}", axum::routing::get(get_vanilla_game))
		.route("/fabric", axum::routing::get(get_fabric))
		.route(
			"/fabric/{game_version}",
			axum::routing::get(get_fabric_loader),
		)
		.route(
			"/fabric/{game_version}/{loader_version}",
			axum::routing::get(get_fabric_installer),
		)
		.route(
			"/fabric/{game_version}/{loader_version}/{installer_version}",
			axum::routing::get(get_fabric_game),
		)
		.route("/paper", axum::routing::get(get_paper))
		.route(
			"/paper/{game_version}",
			axum::routing::get(get_paper_loader),
		)
		.route(
			"/paper/{game_version}/{loader_version}",
			axum::routing::get(get_paper_game),
		)
}

pub async fn get() -> impl IntoResponse {
	(
		StatusCode::OK,
		Json(vec![
			"vanilla".to_string(),
			"fabric".to_string(),
			"paper".to_string(),
		]),
	)
		.into_response()
}

// Vanilla Minecraft Java

pub async fn get_vanilla(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let versions_res = state.binary_service.mcje.list_versions().await;

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

pub async fn get_vanilla_game(
	State(state): State<Arc<AppState>>,
	Path(version): Path<String>,
) -> impl IntoResponse {
	let versions_res = state.binary_service.mcje.list_versions().await;

	if let Err(err) = versions_res {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Internal server error: {}", err),
		)
			.into_response();
	}

	let versions = versions_res.unwrap();

	if !versions.contains(&version) {
		return (
			StatusCode::NOT_FOUND,
			format!("Version {version} not found"),
		)
			.into_response();
	}

	(
		StatusCode::OK,
		Json(Game::MinecraftJava(MinecraftJava {
			loader: MinecraftJavaLoader::Vanilla,
			version,
		})),
	)
		.into_response()
}

// Fabric Minecraft Java

pub async fn get_fabric(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let versions_res = state.binary_service.fabric.list_game_versions().await;

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

pub async fn get_fabric_loader(
	State(state): State<Arc<AppState>>,
	Path(game_version): Path<String>,
) -> impl IntoResponse {
	let versions_res = state
		.binary_service
		.fabric
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

pub async fn get_fabric_installer(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let versions_res = state.binary_service.fabric.list_installer_versions().await;

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

pub async fn get_fabric_game(
	Path((game_version, loader_version, installer_version)): Path<(String, String, String)>,
) -> impl IntoResponse {
	let game = Game::MinecraftJava(MinecraftJava {
		version: game_version,
		loader: MinecraftJavaLoader::Fabric {
			loader: loader_version,
			launcher: installer_version,
		},
	});

	(StatusCode::OK, Json(game)).into_response()
}

// Paper Minecraft Java

pub async fn get_paper(State(state): State<Arc<AppState>>) -> impl IntoResponse {
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

pub async fn get_paper_loader(
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

pub async fn get_paper_game(
	Path((game_version, loader_version)): Path<(String, u16)>,
) -> impl IntoResponse {
	let game = Game::MinecraftJava(MinecraftJava {
		version: game_version,
		loader: MinecraftJavaLoader::Paper {
			build: loader_version,
		},
	});

	(StatusCode::OK, Json(game)).into_response()
}
