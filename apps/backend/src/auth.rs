use axum::{extract::Request, response::Response};
use base64::engine::general_purpose;
use base64::Engine;
use futures_util::future::BoxFuture;
use jsonwebtoken::EncodingKey;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use std::time::Duration;
use tower::{Layer, Service};
use tower_cookies::Cookies;

static AUTH_TOKEN_LENGTH: Duration = Duration::from_secs(60 * 5); // 5 minutes

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenClaims {
	pub iat: u64,
	pub exp: u64,
	pub sub: String,
}

pub fn create_auth_token(user_id: String) -> String {
	let time_now = std::time::SystemTime::now();
	let issued_at_secs = time_now
		.duration_since(std::time::UNIX_EPOCH)
		.expect("Failed to convert to UNIX timestamp")
		.as_secs();
	let expiration_secs = time_now
		.checked_add(AUTH_TOKEN_LENGTH)
		.expect("Failed to calculate expiration time")
		.duration_since(std::time::UNIX_EPOCH)
		.expect("Failed to convert to UNIX timestamp")
		.as_secs();

	let auth_jwt_claims = AuthTokenClaims {
		iat: issued_at_secs,
		exp: expiration_secs,
		sub: user_id,
	};

	jsonwebtoken::encode(
		&jsonwebtoken::Header::default(),
		&auth_jwt_claims,
		&EncodingKey::from_secret(b"hunter2"), // TODO: Make randomized secret
	)
	.expect("Failed to create auth token")
}

pub fn create_refresh_token() -> String {
	let mut bytes = [0u8; 32];
	rand::rng().fill(&mut bytes);
	general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

// TODO: Use tower layer to extract user, create a middleware to ensure user is present (authenticated).
// https://docs.rs/tower/latest/tower/trait.Layer.html

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
	type Service = AuthMiddleware<S>;

	fn layer(&self, inner: S) -> Self::Service {
		AuthMiddleware { inner }
	}
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
	inner: S,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
	S: Service<Request, Response = Response> + Send + 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, request: Request) -> Self::Future {
		let cookies = request
			.extensions()
			.get::<Cookies>()
			.expect("CookieManagerLayer must be applied before AuthLayer");

		// TODO: Validate the auth token and extract user information.

		let future = self.inner.call(request);
		Box::pin(async move {
			let response: Response = future.await?;
			Ok(response)
		})
	}
}
