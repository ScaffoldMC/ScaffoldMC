mod api;
mod config;
mod core;
mod db;
mod services;

use core::logger::Logger;
use core::secrets::Secrets;
use db::Database;
use log::{info, LevelFilter};
use services::binary::BinaryService;
use services::server::ServerService;
use std::sync::Arc;
use std::{env, net::SocketAddr};

use crate::services::auth::AuthService;

static LOGGER: Logger = Logger;

#[derive(Clone)]
struct AppState {
	pub server_service: Arc<ServerService>,
	pub auth_service: Arc<AuthService>,
	pub binary_service: Arc<BinaryService>,
}

impl AppState {
	pub async fn new() -> Self {
		let base_dir = env::current_dir()
			.expect("Current dir should be accessible")
			.join("data/");

		if !base_dir.exists() {
			std::fs::create_dir_all(&base_dir).expect("Read/write should be available");
		}

		let db = Arc::new(
			Database::new(&base_dir.join("db.sqlite"))
				.await
				.expect("Failed to start DB"),
		);

		let secrets = Secrets::new(&base_dir);

		let binary_service = Arc::new(BinaryService::new());

		AppState {
			server_service: Arc::new(ServerService::new(
				"data/servers".into(),
				binary_service.clone(),
			)),
			auth_service: Arc::new(AuthService::new(db, secrets)),
			binary_service,
		}
	}
}

#[tokio::main]
async fn main() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
		.expect("Failed to set logger");

	let state = Arc::new(AppState::new().await);
	let app = api::routes::create_router(state);
	let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

	info!("Starting server on {}", addr);
	axum_server::bind(addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
