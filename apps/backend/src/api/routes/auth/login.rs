use std::sync::Arc;

use axum::Router;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json};
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::api::types::auth::LoginRequest;
use crate::config::{AUTH_COOKIE_NAME, REFRESH_COOKIE_NAME};
use crate::services::auth::AuthServiceError;
use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new().route("/", routing::post(post))
}

#[utoipa::path(
	post,
	path = "/auth/login",
	request_body = LoginRequest,
	responses(
		(status = 200, description = "User logged in successfully"),
		(status = 401, description = "Invalid credentials"),
		(status = 500, description = "Internal server error"),
	)
)]
async fn post(
	cookies: Cookies,
	State(state): State<Arc<AppState>>,
	Json(creds): Json<LoginRequest>,
) -> impl IntoResponse {
	let (auth_token, ref_token) = match state
		.auth_service
		.authenticate_user(&creds.username, &creds.password)
		.await
	{
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
		},
	};

	let refresh_cookie = Cookie::build((REFRESH_COOKIE_NAME, ref_token.clone()))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(refresh_cookie);

	let auth_cookie = Cookie::build((AUTH_COOKIE_NAME, auth_token.clone()))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(auth_cookie);

	StatusCode::OK.into_response()
}
