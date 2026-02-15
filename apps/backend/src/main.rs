#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod core;
mod db;
mod services;
mod util;

use config::CLIENT_USER_AGENT;
use core::secrets::Secrets;
use db::Database;
use services::binary::BinaryService;
use services::server::ServerService;
use std::sync::Arc;
use std::{env, net::SocketAddr};

use crate::services::auth::AuthService;
use crate::services::user::UserService;

#[derive(Clone)]
struct AppState {
	pub server_service: Arc<ServerService>,
	pub auth_service: Arc<AuthService>,
	pub binary_service: Arc<BinaryService>,
	pub user_service: Arc<UserService>,
	pub reqwest_client: reqwest::Client,
}

impl AppState {
	pub async fn new() -> Self {
		let base_dir = env::current_dir()
			.expect("Current dir should be accessible")
			.join(config::DATA_FOLDER);

		tracing::info!("Using base data directory: {:?}", base_dir);

		if !base_dir.exists() {
			tracing::info!(
				"Base data directory {:?} does not exist, creating it now",
				base_dir
			);

			std::fs::create_dir_all(&base_dir).expect("Unable to create base data directory.");
		}

		let db = Arc::new(
			Database::new(&base_dir.join("db.sqlite"))
				.await
				.expect("Failed to start DB"),
		);

		let secrets = Secrets::new(&base_dir);

		let reqwest_client = reqwest::Client::builder()
			.user_agent(CLIENT_USER_AGENT)
			.build()
			.expect("Failed to create reqwest client");

		let binary_service = Arc::new(BinaryService::new(reqwest_client.clone()));
		let user_service = Arc::new(UserService::new(db.clone()));

		AppState {
			server_service: Arc::new(ServerService::new(binary_service.clone())),
			auth_service: Arc::new(AuthService::new(db, secrets)),
			binary_service,
			user_service,
			reqwest_client,
		}
	}
}

#[tokio::main]
async fn main() {
	let tracing_subscriber = tracing_subscriber::fmt()
		.compact()
		.with_max_level(tracing::Level::INFO)
		.with_target(false)
		.finish();

	tracing::subscriber::set_global_default(tracing_subscriber)
		.expect("Failed to set tracing subscriber.");

	let state = Arc::new(AppState::new().await);
	let app = api::routes::create_router(state);
	let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

	tracing::info!("Starting server on {}", addr);
	axum_server::bind(addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
