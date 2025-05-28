use axum::{routing::get, Router};

use crate::db;

pub fn create_router() -> Router<db::Database> {
	Router::new().route("/", get(|| async { "Hello, World!" }))
}
