use axum::{routing::get, Router};

pub fn create_router() -> Router {
	Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }))
}
