use crate::{
	api::types::server::{ConsoleQueryParams, ServerCommandRequest},
	core::server::{Server, ServerStateInfo},
	AppState,
};
use axum::{
	extract::{Query, State},
	response::{
		sse::{Event, KeepAlive},
		IntoResponse, Sse,
	},
	routing, Extension, Json, Router,
};
use futures_util::{stream, Stream};
use reqwest::StatusCode;
use std::{sync::Arc, time::Duration};

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/", routing::get(get))
		.route("/", routing::post(post))
}

async fn get(
	State(state): State<Arc<AppState>>,
	Extension(server): Extension<Arc<Server>>,
	Query(query): Query<ConsoleQueryParams>,
) -> Sse<impl Stream<Item = Result<Event, String>>> {
	let stream = stream::unfold(
		(state, server.clone(), query.since),
		|(state, server, mut since)| async move {
			let mut prev_server_state: Option<ServerStateInfo> = None;

			loop {
				let server_info = server.get_server_info().await;

				if prev_server_state.is_none() {
					prev_server_state.replace(server_info.state.clone());
				} else if prev_server_state == Some(ServerStateInfo::Stopped)
					&& server_info.state != ServerStateInfo::Stopped
				{
					// If state has changed since from stop to start, end the stream
					tracing::info!("Server {} has restarted, ending console stream", server.id);
					return None;
				}

				prev_server_state.replace(server_info.state.clone());

				// Get the next snapshot of console lines
				let snapshot = match server.get_console_snapshot(since).await {
					Ok(snapshot) => snapshot,
					Err(err) => {
						tracing::error!(
							"Error getting console stream for server {}: {}",
							server.id,
							err
						);
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
							server.id,
							err
						);

						let event = Event::default()
							.event("error")
							.data("Failed to serialize console snapshot");

						return Some((Ok(event), (state, server, since)));
					}
				};

				let event = Event::default()
					.event("console")
					.id(last_num.to_string())
					.data(payload);

				return Some((Ok(event), (state, server, since)));
			}
		},
	);

	Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn post(
	Extension(server): Extension<Arc<Server>>,
	Json(request): Json<ServerCommandRequest>,
) -> impl IntoResponse {
	match server.send_command(&request.command).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(err) => {
			tracing::error!("Error sending command to server {}: {}", server.id, err);
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}
