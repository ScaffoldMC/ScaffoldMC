use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use log::error;
use password_auth::{verify_password, VerifyError};
use serde::Deserialize;
use tokio::task::spawn_blocking;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::auth;
use crate::db;

static REFRESH_COOKIE_NAME: &str = "refresh_token";

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

pub fn create_router() -> Router<db::Database> {
	Router::new()
		.route("/login", post(login))
		.route("/logout", post(logout))
		.route("/refresh", post(refresh))
}

pub async fn login(
	cookies: Cookies,
	State(db): State<db::Database>,
	Json(creds): Json<Credentials>,
) -> impl IntoResponse {
	let user = db.get_user_by_username(creds.username.as_str()).await;
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

	db.add_refresh_token(&refresh_token, user.id)
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

pub async fn refresh(cookies: Cookies, State(db): State<db::Database>) -> impl IntoResponse {
	// TODO: Check token is in DB, if so then drop if not then 401

	let new_refresh_token = auth::create_refresh_token();

	// TODO: Add refresh token to DB

	let new_cookie = Cookie::build((REFRESH_COOKIE_NAME, new_refresh_token))
		.path("/")
		.http_only(true)
		.secure(true)
		.same_site(tower_cookies::cookie::SameSite::Strict)
		.build();

	cookies.add(new_cookie);

	StatusCode::OK.into_response() // TODO: Return new auth token
}

pub async fn logout(cookies: Cookies, State(db): State<db::Database>) -> impl IntoResponse {
	let refresh_token = cookies
		.get(REFRESH_COOKIE_NAME)
		.map(|cookie| cookie.value().to_string());

	if let Some(ref_token) = refresh_token {
		if let Err(e) = db.delete_refresh_token(&ref_token).await {
			error!("Failed to delete refresh token: {}", e);
			return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response();
		}
	}

	cookies.remove(Cookie::build(REFRESH_COOKIE_NAME).build());

	StatusCode::OK.into_response()
}
