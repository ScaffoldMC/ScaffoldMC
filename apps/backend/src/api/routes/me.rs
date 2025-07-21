use crate::db::user::User;
use crate::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing, Extension, Json, Router};
use std::sync::Arc;

use crate::api::types::user::UserResponse;

impl From<User> for UserResponse {
	fn from(db_user: User) -> Self {
		UserResponse {
			id: db_user.id,
			fullname: db_user.fullname,
			username: db_user.username,
		}
	}
}

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::get(get))
}

pub async fn get(Extension(user): Extension<User>) -> impl IntoResponse {
	(StatusCode::OK, Json(UserResponse::from(user))).into_response()
}
