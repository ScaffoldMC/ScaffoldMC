mod auth;
mod servers;

use crate::auth::require_auth;
use crate::AppState;
use axum::{middleware, Router};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

pub fn create_router(state: Arc<AppState>) -> Router {
	Router::new()
		.nest("/servers", servers::create_router())
		.route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
		.nest("/auth", auth::create_router())
		.layer(CookieManagerLayer::new())
		.with_state(state)
}
