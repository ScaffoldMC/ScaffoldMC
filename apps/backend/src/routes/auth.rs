use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use axum_messages::Messages;

use crate::backend::{AuthSession, Credentials};

pub fn create_router() -> Router {
	Router::new()
		.route("/login", post(login))
		.route("/logout", post(logout))
}

pub async fn login(
	mut auth_session: AuthSession,
	messages: Messages,
	Json(creds): Json<Credentials>,
) -> impl IntoResponse {
	let user = match auth_session.authenticate(creds.clone()).await {
		Ok(Some(user)) => user,
		Ok(None) => {
			messages.error("Invalid username or password");
			return StatusCode::UNAUTHORIZED.into_response();
		}
		Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
	};

	if auth_session.login(&user).await.is_err() {
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}

	messages.success(format!("Successfully logged in as {}", user.username()));

	StatusCode::OK.into_response()
}

async fn logout() {
	todo!("Logout logic");
}
