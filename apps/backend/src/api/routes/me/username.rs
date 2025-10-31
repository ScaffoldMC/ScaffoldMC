use crate::AppState;
use crate::{api::types::user::UsernameChangeRequest, db::user::User};
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, routing, Extension, Json, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::post(post))
}

pub async fn post(
	Extension(user): Extension<User>,
	State(state): State<Arc<AppState>>,
	Json(req): Json<UsernameChangeRequest>,
) -> impl IntoResponse {
	let db_res = state
		.user_service
		.change_username(&user, &req.new_username)
		.await;

	if let Err(err) = db_res {
		match err {
			crate::services::user::UserServiceError::UsernameTaken => {
				return (StatusCode::CONFLICT, "Username already taken").into_response();
			}
			crate::services::user::UserServiceError::ServerError(_) => {
				return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
					.into_response();
			}
		}
	}

	(StatusCode::OK, "Username changed successfully").into_response()
}
