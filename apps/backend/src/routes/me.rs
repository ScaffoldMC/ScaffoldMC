use crate::db::user::User;
use crate::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;
use std::sync::Arc;
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS, Serialize)]
#[ts(export)]
struct UserResponse {
	id: Uuid,
	fullname: String,
	username: String,
}

impl UserResponse {
	pub fn from(db_user: User) -> Self {
		UserResponse {
			id: db_user.id,
			fullname: db_user.fullname,
			username: db_user.username,
		}
	}
}

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", get(me))
}

pub async fn me(Extension(user): Extension<User>) -> impl IntoResponse {
	(StatusCode::OK, Json(UserResponse::from(user))).into_response()
}
