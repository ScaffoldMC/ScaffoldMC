mod backend;
mod logger;
mod routes;
mod user;

use std::{env, net::SocketAddr, path::PathBuf};

use axum::Router;
use axum_login::{
	tower_sessions::{MemoryStore, SessionManagerLayer},
	AuthManagerLayerBuilder,
};
use axum_messages::MessagesManagerLayer;
use log::{info, LevelFilter};
use std::sync::OnceLock;

static LOGGER: logger::Logger = logger::Logger;
static BASE_DIR: OnceLock<PathBuf> = OnceLock::new();

#[tokio::main]
async fn main() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
		.expect("Failed to set logger");

	let base_dir = BASE_DIR.get_or_init(|| env::current_dir().unwrap().join("data/"));

	if !base_dir.exists() {
		std::fs::create_dir_all(base_dir).expect("Failed to create base directory");
	}

	let db_connect_options = sqlx::sqlite::SqliteConnectOptions::new()
		.filename(base_dir.join("db.sqlite"))
		.create_if_missing(true);

	let db = sqlx::SqlitePool::connect_with(db_connect_options)
		.await
		.unwrap();

	let session_store = MemoryStore::default();
	let session_layer = SessionManagerLayer::new(session_store);
	let backend = backend::Backend::new(db);
	let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

	let app: Router = routes::create_router()
		.layer(MessagesManagerLayer)
		.layer(auth_layer);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

	info!("Starting server on {}", addr);
	axum_server::bind(addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
