use axum::{routing::get, Router};

use super::RouterWithState;

pub fn create_router() -> RouterWithState {
	Router::new().route("/", get(|| async { "Hello, World!" }))
}
