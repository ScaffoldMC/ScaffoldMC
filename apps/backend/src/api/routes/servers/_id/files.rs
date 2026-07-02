use crate::models::file_manager::types::{FSEntry, FileManagerError};
use crate::models::server::Server;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Path, Request};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::{routing, Extension, Json, Router};
use futures_util::TryStreamExt;
use reqwest::StatusCode;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{copy, AsyncWriteExt};
use tokio_util::io::{ReaderStream, StreamReader};

pub fn create_router() -> Router<Arc<AppState>> {
	Router::new()
		.route(
			"/{*path}",
			routing::get(get).post(post).delete(delete).put(put),
		)
		.route("/", routing::get(get_root).put(put_root))
}

fn handle_error(error: FileManagerError) -> impl IntoResponse {
	match error {
		FileManagerError::NoPermission => {
			(StatusCode::BAD_REQUEST, error.to_string()).into_response()
		}
		FileManagerError::UnknownType
		| FileManagerError::EncodingError
		| FileManagerError::IoError(_) => {
			tracing::error!("{}", error.to_string());
			(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
		}
		FileManagerError::NotFound => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
	}
}

async fn post(
	Path((_, file_path)): Path<(String, String)>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);

	if path_buf.exists() {
		return (
			StatusCode::BAD_REQUEST,
			"A file or directory already exists at the provided path",
		)
			.into_response();
	}

	if path_buf.ends_with("/") {
		if let Err(err) = file_manager.create_dir(&path_buf).await {
			tracing::error!("Failed to create directory: {}", err);
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to create directory",
			)
				.into_response();
		}
	} else {
		if let Err(err) = file_manager.create_file(&path_buf).await {
			tracing::error!("Failed to create file: {}", err);
			return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create file").into_response();
		}
	}

	StatusCode::CREATED.into_response()
}

async fn get_root(Extension(server): Extension<Arc<Server>>) -> impl IntoResponse {
	get_handler(server, "").await.into_response()
}

async fn get(
	Path((_, file_path)): Path<(String, String)>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	get_handler(server, &file_path).await.into_response()
}

async fn get_handler(server: Arc<Server>, file_path: &str) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);
	let path_stat = file_manager.stat(&path_buf).await;

	if let Err(err) = path_stat {
		return handle_error(err).into_response();
	}

	let path_stat = path_stat.unwrap();

	match path_stat {
		FSEntry::File(_) => {
			let file_content = file_manager.read_file(&path_buf).await;

			if let Err(err) = file_content {
				return handle_error(err).into_response();
			}

			let file_content = file_content.unwrap();

			let stream = ReaderStream::new(file_content);

			Response::builder()
				.status(StatusCode::OK)
				.header("Content-Type", "application/octet-stream")
				.body(Body::from_stream(stream))
				.unwrap()
				.into_response()
		}
		FSEntry::Dir(_) => {
			let dir_content = file_manager.list_dir(&path_buf).await;

			if let Err(err) = dir_content {
				return handle_error(err).into_response();
			}

			let dir_content = dir_content.unwrap();

			(StatusCode::OK, Json(dir_content)).into_response()
		}
	}
}

async fn delete(
	Path((_, file_path)): Path<(String, String)>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);

	if let Err(err) = file_manager.delete(&path_buf).await {
		return handle_error(err).into_response();
	}

	StatusCode::NO_CONTENT.into_response()
}

async fn put_root(
	Extension(server): Extension<Arc<Server>>,
	request: Request,
) -> impl IntoResponse {
	put_handler("".into(), server, request.into_body())
		.await
		.into_response()
}

async fn put(
	Path((_, file_path)): Path<(String, String)>,
	Extension(server): Extension<Arc<Server>>,
	request: Request,
) -> impl IntoResponse {
	put_handler(file_path, server, request.into_body())
		.await
		.into_response()
}

async fn put_handler(file_path: String, server: Arc<Server>, req_body: Body) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);

	let file_writer = file_manager.write_file(&path_buf).await;

	if let Err(err) = file_writer {
		return handle_error(err).into_response();
	}

	let mut file_writer = file_writer.unwrap();

	let body_stream = req_body
		.into_data_stream()
		.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));

	let mut body_reader = StreamReader::new(body_stream);

	if let Err(err) = copy(&mut body_reader, &mut file_writer).await {
		tracing::error!("Error while copying body to file: {}", err);
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}

	if let Err(err) = file_writer.flush().await {
		tracing::error!("Error while flushing file writer: {}", err);
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}
	StatusCode::OK.into_response()
}
