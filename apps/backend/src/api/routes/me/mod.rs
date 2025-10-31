use crate::AppState;
use crate::{api::middleware::auth::require_sudo, db::user::User};
use axum::middleware;
use axum::{http::StatusCode, response::IntoResponse, routing, Extension, Json, Router};
use std::sync::Arc;

use crate::api::types::user::UserResponse;

mod username;

pub fn create_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.nest("/username", username::create_router())
		.route_layer(middleware::from_fn_with_state(state.clone(), require_sudo))
		.route("/", routing::get(get))
}

pub async fn get(Extension(user): Extension<User>) -> impl IntoResponse {
	(StatusCode::OK, Json(UserResponse::from(user))).into_response()
}
