use std::time::Instant;

use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tracing::{Instrument, Span};

pub async fn log_request(req: Request<Body>, next: Next) -> Response {
	let span = tracing::info_span!(
		"API",
		method = %req.method(),
		uri = %req.uri(),
		status = tracing::field::Empty,
	);

	let start = Instant::now();

	async move {
		let response = next.run(req).await;
		let elapsed = start.elapsed();

		Span::current().record("status", response.status().as_u16());

		tracing::info!("request completed (took {:.2?})", elapsed);

		response
	}
	.instrument(span)
	.await
}
