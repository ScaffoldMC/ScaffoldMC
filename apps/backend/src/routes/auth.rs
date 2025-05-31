use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use log::error;
use password_auth::{verify_password, VerifyError};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::auth;
use crate::auth::REFRESH_TOKEN_LENGTH;
use crate::AppState;

static REFRESH_COOKIE_NAME: &str = "refresh_token";

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/login", post(login))
		.route("/logout", post(logout))
		.route("/refresh", post(refresh))
}

pub async fn login(
	cookies: Cookies,
	State(state): State<Arc<AppState>>,
	Json(creds): Json<Credentials>,
) -> impl IntoResponse {
	let user = state.db.get_user_by_username(creds.username.as_str()).await;
	if let Err(_) = user {
		// Handle error, e.g., user not found or database error.
		return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
	}
	let user = user.unwrap();

	let verify_result = spawn_blocking(move || {
		verify_password(creds.password.as_str(), user.password_hash.as_str())
	})
	.await
	.expect("Join error on spawn_blocking task");

	if let Err(err) = verify_result {
		return match err {
			VerifyError::PasswordInvalid => {
				(StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
			}
			VerifyError::Parse(e) => {
				error!("Failed to parse password hash: {}", e);
				(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
			}
		};
	}

	let auth_token = auth::create_auth_token(user.id.to_string());
	let refresh_token = auth::create_refresh_token();

	state
		.db
		.add_refresh_token(&refresh_token, user.id)
		.await
		.expect("Failed to add refresh token");

	let refresh_cookie = Cookie::build((REFRESH_COOKIE_NAME, refresh_token))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(refresh_cookie);
	(StatusCode::OK, Json(auth_token)).into_response()
}

#[derive(Serialize)]
struct RefreshResponseBody {
	pub ref_token: String,
	pub auth_token: String,
}

pub async fn refresh(cookies: Cookies, State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let ref_token = cookies
		.get(REFRESH_COOKIE_NAME)
		.map(|cookie| cookie.value().to_string());

	if ref_token.is_none() {
		return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
	}

	let ref_token = ref_token.unwrap();

	let db_entry = state.db.get_refresh_token(&ref_token).await;

	if let Err(err) = db_entry {
		error!("Failed to get refresh token from DB: {}", err);
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}

	let db_entry = db_entry.unwrap();

	if db_entry.is_none() {
		return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
	}

	let db_entry = db_entry.unwrap();

	let current_time = time::OffsetDateTime::now_utc();
	let token_age = current_time - db_entry.created_at;

	if token_age > REFRESH_TOKEN_LENGTH {
		state
			.db
			.delete_refresh_token(&ref_token)
			.await
			.expect("Failed to delete refresh token");

		return (StatusCode::UNAUTHORIZED, "Token expired").into_response();
	}

	let auth_token = auth::create_auth_token(db_entry.user_id.to_string());
	let new_ref_token = auth::create_refresh_token();

	if let Err(err) = state
		.db
		.add_refresh_token(&new_ref_token, db_entry.user_id)
		.await
	{
		error!("Failed to store refresh token in DB: {}", err);
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}

	let new_cookie = Cookie::build((REFRESH_COOKIE_NAME, new_ref_token))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(new_cookie);

	(
		StatusCode::OK,
		Json(RefreshResponseBody {
			ref_token,
			auth_token,
		}),
	)
		.into_response()
}

pub async fn logout(cookies: Cookies, State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let refresh_token = cookies
		.get(REFRESH_COOKIE_NAME)
		.map(|cookie| cookie.value().to_string());

	if let Some(ref_token) = refresh_token {
		if let Err(e) = state.db.delete_refresh_token(&ref_token).await {
			error!("Failed to delete refresh token: {}", e);
			return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response();
		}
	}

	cookies.remove(Cookie::build(REFRESH_COOKIE_NAME).build());

	StatusCode::OK.into_response()
}
