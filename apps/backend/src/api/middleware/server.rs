use crate::{services::server::ServerServiceError, AppState};
use axum::{
	body::Body,
	extract::{Path, Request, State},
	middleware::Next,
	response::Response,
};
use reqwest::StatusCode;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub async fn require_server(
	State(state): State<Arc<AppState>>,
	Path(params): Path<HashMap<String, String>>,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response, StatusCode> {
	let id = params
		.get("id")
		.and_then(|s| Uuid::parse_str(s).ok())
		.ok_or(StatusCode::BAD_REQUEST)?;

	let server = match state.server_service.get_server(id).await {
		Ok(server) => server,
		Err(err) => match err {
			ServerServiceError::NoSuchServer(_) => {
				return Err(StatusCode::NOT_FOUND);
			}
			err => {
				tracing::error!("Error getting server: {}", err);
				return Err(StatusCode::INTERNAL_SERVER_ERROR);
			}
		},
	};

	req.extensions_mut().insert(server);
	Ok(next.run(req).await)
}
