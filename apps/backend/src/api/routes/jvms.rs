use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing, Json, Router};

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

/// List the installed JVMs
pub async fn get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	match state.java_service.get_versions().await {
		Ok(jvm_versions) => (axum::http::StatusCode::OK, Json(jvm_versions)).into_response(),
		Err(e) => {
			tracing::error!("Error fetching JVM versions: {}", e);

			(
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				format!("Internal server error: {e}"),
			)
				.into_response()
		}
	}
}
