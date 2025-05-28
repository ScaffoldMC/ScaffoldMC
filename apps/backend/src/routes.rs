mod auth;
mod servers;

use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::db;

pub type RouterWithState = Router<db::Database>;

pub fn create_router() -> RouterWithState {
	Router::new()
		.nest("/servers", servers::create_router())
		// TODO: Create auth middleware
		// .route_layer(auth_layer)
		.nest("/auth", auth::create_router())
}
