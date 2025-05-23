mod auth;
mod servers;

use axum::Router;
use axum_login::login_required;

use crate::backend::Backend;

pub fn create_router() -> Router {
	Router::new()
		.nest("/servers", servers::create_router())
		.route_layer(login_required!(Backend, login_url = "/auth/login"))
		.nest("/auth", auth::create_router())
}
