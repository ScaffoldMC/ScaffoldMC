use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json};
use axum::{Extension, Router};
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::api::types::auth::{SudoCheckResponse, SudoRequest};
use crate::config::AUTH_COOKIE_NAME;
use crate::db::user::User;
use crate::services::auth::AuthServiceError;
use crate::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::post(post))
		.route("/", routing::get(get))
}

async fn get(cookies: Cookies, State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let auth_cookie = cookies.get(AUTH_COOKIE_NAME);

	if auth_cookie.is_none() {
		return StatusCode::UNAUTHORIZED.into_response();
	}

	let is_sudo = state
		.auth_service
		.token_is_sudo(&auth_cookie.unwrap().value())
		.await;

	if let Err(err) = is_sudo {
		return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
	}

	(
		StatusCode::OK,
		Json(SudoCheckResponse {
			sudo: is_sudo.unwrap(),
		}),
	)
		.into_response()
}

async fn post(
	cookies: Cookies,
	Extension(user): Extension<User>,
	State(state): State<Arc<AppState>>,
	Json(creds): Json<SudoRequest>,
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
