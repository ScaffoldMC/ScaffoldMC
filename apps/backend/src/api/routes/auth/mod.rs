use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod login;
mod logout;
mod refresh;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.nest("/login", login::create_router())
		.nest("/logout", logout::create_router())
		.nest("/refresh", refresh::create_router())
}
