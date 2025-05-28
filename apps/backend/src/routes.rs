mod auth;
mod servers;

use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::db;

pub fn create_router() -> Router<db::Database> {
	Router::new()
		.nest("/servers", servers::create_router())
		// TODO: Create auth middleware
		// .route_layer(auth_layer)
		.nest("/auth", auth::create_router())
}
