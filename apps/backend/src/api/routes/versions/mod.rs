mod mcje;

use crate::AppState;
use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", axum::routing::get(get_games))
		.nest("/minecraft-java", mcje::create_router())
}

pub async fn get_games() -> impl IntoResponse {
	(StatusCode::OK, Json(vec!["minecraft-java"])).into_response()
}
