mod auth;
mod binaries;
mod jvms;
mod me;
mod servers;

use crate::api::middleware::auth::require_auth;
use crate::AppState;
use axum::{http, middleware, Router};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub fn create_router(state: Arc<AppState>) -> Router {
	let cors = CorsLayer::new()
		.allow_origin(
			"http://localhost:3000"
				.parse::<http::HeaderValue>()
				.unwrap(),
		)
		.allow_methods([
			http::Method::GET,
			http::Method::POST,
			http::Method::PUT,
			http::Method::DELETE,
		])
		.allow_headers([http::header::CONTENT_TYPE])
		.allow_credentials(true);

	Router::new()
		.nest("/servers", servers::create_router())
		.nest("/binaries", binaries::create_router())
		.nest("/me", me::create_router())
		.nest("/jvms", jvms::create_router())
		.route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
		.nest("/auth", auth::create_router(state.clone()))
		.layer(CookieManagerLayer::new())
		.layer(cors)
		.with_state(state)
}
