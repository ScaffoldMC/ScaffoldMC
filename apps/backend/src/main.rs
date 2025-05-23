mod backend;
mod logger;
mod routes;
mod user;

use std::net::SocketAddr;

use axum::Router;
use axum_login::{
	tower_sessions::{MemoryStore, SessionManagerLayer},
	AuthManagerLayerBuilder,
};
use axum_messages::MessagesManagerLayer;
use log::{info, LevelFilter};

static LOGGER: logger::Logger = logger::Logger;

#[tokio::main]
async fn main() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
		.expect("Failed to set logger");

	let db = sqlx::SqlitePool::connect("sqlite://:memory:")
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
