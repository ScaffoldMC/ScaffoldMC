use crate::db::user::User;
use crate::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing, Extension, Json, Router};
use std::sync::Arc;

use crate::api::types::user::UserResponse;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

pub async fn get(Extension(user): Extension<User>) -> impl IntoResponse {
	(StatusCode::OK, Json(UserResponse::from(user))).into_response()
}
