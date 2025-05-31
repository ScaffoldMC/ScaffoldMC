mod auth;
mod db;
mod logger;
mod routes;

use axum::Router;
use db::Database;
use log::{info, LevelFilter};
use std::sync::{Arc, OnceLock};
use std::{env, net::SocketAddr, path::PathBuf};
use tower_cookies::CookieManagerLayer;

static LOGGER: logger::Logger = logger::Logger;
static BASE_DIR: OnceLock<PathBuf> = OnceLock::new();

#[derive(Clone)]
struct AppState {
	pub db: Database,
}

#[tokio::main]
async fn main() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
		.expect("Failed to set logger");

	let base_dir = BASE_DIR.get_or_init(|| env::current_dir().unwrap().join("data/"));

	if !base_dir.exists() {
		std::fs::create_dir_all(base_dir).expect("Failed to create base directory");
	}

	let db = db::Database::new(&base_dir.join("db.sqlite"))
		.await
		.expect("Failed to initialize database");

	let state = Arc::new(AppState { db });

	let app = Router::new()
		.merge(routes::create_router())
		.layer(CookieManagerLayer::new())
		.with_state(state);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

	info!("Starting server on {}", addr);
	axum_server::bind(addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
