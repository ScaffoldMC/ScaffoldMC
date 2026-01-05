mod fabric;
mod paper;
mod vanilla;

use crate::{api::types::versions::OptionsResponse, AppState};
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
	let options = OptionsResponse {
		message: "Select Loader".to_string(),
		options: vec![
			"vanilla".to_string(),
			"fabric".to_string(),
			"paper".to_string(),
		],
	};

	(StatusCode::OK, Json(options)).into_response()
}
