mod fabric;
mod paper;
mod vanilla;

use crate::AppState;
use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", axum::routing::get(get))
		.nest("/vanilla", vanilla::create_router())
		.nest("/paper", paper::create_router())
		.nest("/fabric", fabric::create_router())
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
