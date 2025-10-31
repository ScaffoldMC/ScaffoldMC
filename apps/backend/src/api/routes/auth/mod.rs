use std::sync::Arc;

use axum::{middleware, Router};

use crate::{api::middleware::auth::require_auth, AppState};

mod login;
mod logout;
mod refresh;
mod sudo;

pub fn create_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.nest("/sudo", sudo::create_router())
		.route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
		.nest("/login", login::create_router())
		.nest("/logout", logout::create_router())
		.nest("/refresh", refresh::create_router())
}
