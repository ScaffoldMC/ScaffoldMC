use crate::AppState;
use crate::{api::middleware::auth::require_sudo, db::user::User};
use axum::extract::State;
use axum::middleware;
use axum::{http::StatusCode, response::IntoResponse, routing, Extension, Json, Router};
use std::sync::Arc;

use crate::api::types::user::{UserPatchRequest, UserResponse};

pub fn create_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::patch(patch))
		.route_layer(middleware::from_fn_with_state(state.clone(), require_sudo))
		.route("/", routing::get(get))
}

pub async fn get(Extension(user): Extension<User>) -> impl IntoResponse {
	(StatusCode::OK, Json(UserResponse::from(user))).into_response()
}

pub async fn patch(
	Extension(user): Extension<User>,
	State(state): State<Arc<AppState>>,
	Json(req): Json<UserPatchRequest>,
) -> impl IntoResponse {
	// Change username if provided
	if let Some(username) = req.username {
		let db_res = state.user_service.change_username(&user, &username).await;

		if let Err(err) = db_res {
			match err {
				crate::services::user::UserServiceError::UsernameTaken => {
					return (StatusCode::CONFLICT, "Username already taken").into_response();
				}
				crate::services::user::UserServiceError::ServerError(_) => {
					tracing::error!("Error updating username: {}", err);

					return (
						StatusCode::INTERNAL_SERVER_ERROR,
						"Internal server error updating username",
					)
						.into_response();
				}
			}
		}
	}

	// Change full name if provided
	if let Some(fullname) = req.fullname {
		let db_res = state.user_service.change_full_name(&user, &fullname).await;

		if let Err(_) = db_res {
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Internal server error updating fullname",
			)
				.into_response();
		}
	}

	// Change password if provided
	if let Some(password) = req.password {
		if let Some(new_password) = req.new_password {
			if let Err(_) = state.auth_service.verify_password(&user, &password).await {
				return (StatusCode::UNAUTHORIZED, "Current password is incorrect").into_response();
			}

			let db_res = state
				.user_service
				.change_password(&user, &new_password)
				.await;

			if let Err(_) = db_res {
				return (
					StatusCode::INTERNAL_SERVER_ERROR,
					"Internal server error updating password",
				)
					.into_response();
			}
		} else {
			return (
				StatusCode::BAD_REQUEST,
				"New password must be provided when changing password",
			)
				.into_response();
		}
	}

	let user_updated = state.user_service.get_user_by_id(user.id).await;

	if let Err(_) = user_updated {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Internal server error fetching updated user",
		)
			.into_response();
	}

	(
		StatusCode::OK,
		Json(UserResponse::from(user_updated.unwrap())),
	)
		.into_response()
}
