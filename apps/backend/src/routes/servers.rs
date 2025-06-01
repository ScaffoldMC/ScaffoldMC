use crate::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(|| async { "Hello, World!" }))
}
