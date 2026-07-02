use crate::api::types::server::{
	FilesGetQueryParams, FilesPostQueryParams, FilesPostType, FilesPutOperation,
	FilesPutQueryParams,
};
use crate::models::file_manager::types::{FSEntry, FileManagerError};
use crate::models::server::Server;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Path, Query, Request};
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
	Query(query): Query<FilesPostQueryParams>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);

	match file_manager.stat(&path_buf).await {
		Ok(_) => {
			return (
				StatusCode::BAD_REQUEST,
				"A file or directory already exists at the provided path",
			)
				.into_response()
		}
		Err(FileManagerError::NotFound) => {}
		Err(err) => return handle_error(err).into_response(),
	}

	let result = match query.entry_type {
		FilesPostType::File => file_manager.create_file(&path_buf).await,
		FilesPostType::Directory => file_manager.create_dir(&path_buf).await,
	};

	if let Err(err) = result {
		return handle_error(err).into_response();
	}

	StatusCode::CREATED.into_response()
}

async fn get_root(
	Query(query): Query<FilesGetQueryParams>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	get_handler(server, "", query).await.into_response()
}

async fn get(
	Path((_, file_path)): Path<(String, String)>,
	Query(query): Query<FilesGetQueryParams>,
	Extension(server): Extension<Arc<Server>>,
) -> impl IntoResponse {
	get_handler(server, &file_path, query).await.into_response()
}

async fn get_handler(
	server: Arc<Server>,
	file_path: &str,
	query: FilesGetQueryParams,
) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);

	let path_stat = match file_manager.stat(&path_buf).await {
		Ok(path_stat) => path_stat,
		Err(err) => return handle_error(err).into_response(),
	};

	if query.content.is_some() {
		match path_stat {
			FSEntry::File(_) => {
				let file_content = match file_manager.read_file(&path_buf).await {
					Ok(file_content) => file_content,
					Err(err) => return handle_error(err).into_response(),
				};

				let stream = ReaderStream::new(file_content);

				Response::builder()
					.status(StatusCode::OK)
					.header("Content-Type", "application/octet-stream")
					.body(Body::from_stream(stream))
					.unwrap()
					.into_response()
			}
			FSEntry::Dir(_) => match file_manager.list_dir(&path_buf).await {
				Ok(dir_content) => (StatusCode::OK, Json(dir_content)).into_response(),
				Err(err) => handle_error(err).into_response(),
			},
		}
	} else {
		(StatusCode::OK, Json(path_stat)).into_response()
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
	Query(query): Query<FilesPutQueryParams>,
	Extension(server): Extension<Arc<Server>>,
	request: Request,
) -> impl IntoResponse {
	put_handler("".into(), query, server, request.into_body())
		.await
		.into_response()
}

async fn put(
	Path((_, file_path)): Path<(String, String)>,
	Query(query): Query<FilesPutQueryParams>,
	Extension(server): Extension<Arc<Server>>,
	request: Request,
) -> impl IntoResponse {
	put_handler(file_path, query, server, request.into_body())
		.await
		.into_response()
}

async fn put_handler(
	file_path: String,
	query: FilesPutQueryParams,
	server: Arc<Server>,
	req_body: Body,
) -> impl IntoResponse {
	let file_manager = server.get_fs();
	let path_buf = PathBuf::from(file_path);

	match query.operation {
		FilesPutOperation::Rename | FilesPutOperation::Move => {
			let new_path = match query.to {
				Some(path) => PathBuf::from(path),
				None => {
					return (
						StatusCode::BAD_REQUEST,
						"The 'to' query parameter is required for rename and move operations",
					)
						.into_response()
				}
			};

			if let Err(err) = file_manager.relocate(&path_buf, &new_path).await {
				return handle_error(err).into_response();
			}

			StatusCode::OK.into_response()
		}
		FilesPutOperation::Write => {
			let path_stat = match file_manager.stat(&path_buf).await {
				Ok(path_stat) => path_stat,
				Err(err) => return handle_error(err).into_response(),
			};

			if let FSEntry::Dir(_) = path_stat {
				return (StatusCode::BAD_REQUEST, "Cannot write to a directory").into_response();
			}

			let mut file_writer = match file_manager.write_file(&path_buf).await {
				Ok(file_writer) => file_writer,
				Err(err) => return handle_error(err).into_response(),
			};

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
	}
}
