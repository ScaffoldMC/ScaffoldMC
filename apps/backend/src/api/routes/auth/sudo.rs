use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json};
use axum::{Extension, Router};
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::api::types::auth::LoginRequest;
use crate::config::AUTH_COOKIE_NAME;
use crate::db::user::User;
use crate::services::auth::AuthServiceError;
use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::post(post))
}

async fn post(
	cookies: Cookies,
	Extension(user): Extension<User>,
	State(state): State<Arc<AppState>>,
	Json(creds): Json<LoginRequest>,
) -> impl IntoResponse {
	let sudo_token = match state.auth_service.sudo_user(user, &creds.password).await {
		Ok(tokens) => tokens,
		Err(err) => match err {
			AuthServiceError::InvalidCredentials => {
				return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
			}
			AuthServiceError::Unauthorized => {
				return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
			}
			AuthServiceError::ServerError(err) => {
				return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
			}
			_ => {
				return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
					.into_response();
			}
		},
	};

	let auth_cookie = Cookie::build((AUTH_COOKIE_NAME, sudo_token.clone()))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(auth_cookie);

	StatusCode::OK.into_response()
}
