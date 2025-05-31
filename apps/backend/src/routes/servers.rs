use std::sync::Arc;

use axum::{routing::get, Router};

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(|| async { "Hello, World!" }))
}
