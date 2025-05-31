mod auth;
mod servers;

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.nest("/servers", servers::create_router())
		// TODO: Create auth middleware
		// .route_layer(auth_layer)
		.nest("/auth", auth::create_router())
}
