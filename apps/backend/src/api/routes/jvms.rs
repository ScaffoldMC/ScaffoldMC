use std::sync::Arc;

use axum::{response::IntoResponse, routing, Json, Router};

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

/// List the installed JVMs
pub async fn get() -> impl IntoResponse {
	match crate::core::java::get_versions().await {
		Ok(jvm_versions) => (axum::http::StatusCode::OK, Json(jvm_versions)).into_response(),
		Err(e) => {
			tracing::error!("Error fetching JVM versions: {}", e);

			(
				axum::http::StatusCode::INTERNAL_SERVER_ERROR,
				format!("Internal server error: {}", e),
			)
				.into_response()
		}
	}
}
