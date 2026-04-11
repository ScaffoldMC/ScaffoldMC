use std::{sync::Arc, time::Duration};

use axum::{
	extract::{Path, Query, State},
	response::{
		sse::{Event, KeepAlive},
		IntoResponse, Sse,
	},
	routing, Json, Router,
};
use futures_util::{stream, Stream};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
	api::types::server::{ConsoleQueryParams, ServerCommandRequest},
	core::server::ServerStateInfo,
	services::server::ServerError,
	AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
}

async fn get(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Query(query): Query<ConsoleQueryParams>,
) -> Sse<impl Stream<Item = Result<Event, String>>> {
	let stream = stream::unfold(
		(state, id, query.since),
		|(state, id, mut since)| async move {
			let mut prev_server_state: Option<ServerStateInfo> = None;
			loop {
				let server_info = match state.server_service.get_server_info(id).await {
					Ok(server_info) => server_info,
					Err(err) => {
						tracing::error!("Error getting server info for server {}: {}", id, err);
						tokio::time::sleep(Duration::from_millis(500)).await;
						continue;
					}
				};

				if prev_server_state.is_none() {
					prev_server_state.replace(server_info.state.clone());
				} else if prev_server_state == Some(ServerStateInfo::Stopped)
					&& server_info.state != ServerStateInfo::Stopped
				{
					// If state has changed since from stop to start, end the stream
					tracing::info!("Server {} has restarted, ending console stream", id);
					return None;
				}

				prev_server_state.replace(server_info.state.clone());

				// Get the next snapshot of console lines
				let snapshot = match state.server_service.get_console_snapshot(id, since).await {
					Ok(snapshot) => snapshot,
					Err(ServerError::NoSuchServer(_)) => {
						tracing::warn!("Console stream requested for unknown server {}", id);
						let event = Event::default().event("error").data("Server not found");
						return Some((Ok(event), (state, id, since)));
					}
					Err(err) => {
						tracing::error!("Error getting console stream for server {}: {}", id, err);
						tokio::time::sleep(Duration::from_millis(500)).await;
						continue;
					}
				};

				if snapshot.is_empty() {
					tokio::time::sleep(Duration::from_millis(250)).await;
					continue;
				}

				// Update the "since" parameter to the last line number in the snapshot
				let last_num = snapshot.last().map(|line| line.num).unwrap_or_default();
				since = Some(last_num);

				// Serialize the snapshot to JSON
				let payload = match serde_json::to_string(&snapshot) {
					Ok(payload) => payload,
					Err(err) => {
						tracing::error!(
							"Failed to serialize console snapshot for server {}: {}",
							id,
							err
						);

						let event = Event::default()
							.event("error")
							.data("Failed to serialize console snapshot");

						return Some((Ok(event), (state, id, since)));
					}
				};

				let event = Event::default()
					.event("console")
					.id(last_num.to_string())
					.data(payload);

				return Some((Ok(event), (state, id, since)));
			}
		},
	);

	Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn post(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Json(request): Json<ServerCommandRequest>,
) -> impl IntoResponse {
	match state
		.server_service
		.send_command(id, &request.command)
		.await
	{
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			if let ServerError::NoSuchServer(_) = err {
				tracing::error!("Server not found: {}", id);
				StatusCode::NOT_FOUND.into_response()
			} else {
				tracing::error!("Error sending command to server {}: {}", id, err);
				StatusCode::INTERNAL_SERVER_ERROR.into_response()
			}
		}
	}
}
