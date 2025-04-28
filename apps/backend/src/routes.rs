mod auth;
mod servers;

use axum::Router;

pub fn create_router() -> Router {
	Router::new()
		.nest("/servers", servers::create_router())
		.nest("/auth", auth::create_router())
}
