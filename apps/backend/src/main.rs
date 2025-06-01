mod auth;
mod db;
mod logger;
mod routes;
mod secrets;

use db::Database;
use log::{info, LevelFilter};
use secrets::Secrets;
use std::sync::Arc;
use std::{env, net::SocketAddr};

static LOGGER: logger::Logger = logger::Logger;

#[derive(Clone)]
struct AppState {
	pub db: Database,
	pub secrets: Secrets,
}

impl AppState {
	pub async fn new() -> Self {
		let base_dir = env::current_dir()
			.expect("Current dir should be accessible")
			.join("data/");

		if !base_dir.exists() {
			std::fs::create_dir_all(&base_dir).expect("Read/write should be available");
		}

		let db = db::Database::new(&base_dir.join("db.sqlite"))
			.await
			.expect("Failed to start DB");

		let secrets = Secrets::new(&base_dir);

		AppState { db, secrets }
	}
}

#[tokio::main]
async fn main() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
		.expect("Failed to set logger");

	let state = Arc::new(AppState::new().await);
	let app = routes::create_router(state);
	let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

	info!("Starting server on {}", addr);
	axum_server::bind(addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
