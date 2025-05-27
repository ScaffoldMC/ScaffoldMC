mod auth;
mod servers;

use axum::Router;
use tower_cookies::CookieManagerLayer;

pub fn create_router() -> Router {
	Router::new()
		.nest("/servers", servers::create_router())
		// TODO: Create auth middleware
		// .route_layer(auth_layer)
		.nest("/auth", auth::create_router())
		.layer(CookieManagerLayer::new())
}
