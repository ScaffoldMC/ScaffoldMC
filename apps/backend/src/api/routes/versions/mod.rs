mod mcje;

use crate::{api::types::versions::OptionsResponse, AppState};
use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", axum::routing::get(get_games))
		.nest("/minecraft-java", mcje::create_router())
}

pub async fn get_games() -> impl IntoResponse {
	let options = OptionsResponse {
		message: "Select Game".to_string(),
		options: vec!["minecraft-java".to_string()],
	};

	(StatusCode::OK, Json(options)).into_response()
}
