use crate::{
	api::types::versions::{CompleteVersionResponse, OptionsResponse},
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
		.route("/{version}", axum::routing::get(get_game))
}

pub async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let versions_res = state.binary_service.mcje.list_versions().await;

	if let Err(err) = versions_res {
		tracing::error!("Error fetching game versions: {}", err);

		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Internal server error: {}", err),
		)
			.into_response()
	} else {
		let options = OptionsResponse {
			message: "Select Version".to_string(),
			options: versions_res.unwrap(),
		};

		(StatusCode::OK, Json(options)).into_response()
	}
}

pub async fn get_game(
	State(state): State<Arc<AppState>>,
	Path(version): Path<String>,
) -> impl IntoResponse {
	let versions_res = state.binary_service.mcje.list_versions().await;

	if let Err(err) = versions_res {
		tracing::error!("Error fetching game versions: {}", err);

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

	let game = Game::MinecraftJava(MinecraftJava {
		loader: MinecraftJavaLoader::Vanilla,
		version,
	});

	let res = CompleteVersionResponse { game };

	(StatusCode::OK, Json(res)).into_response()
}
