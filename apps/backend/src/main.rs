mod logger;
mod routes;

use axum::Router;
use log::{info, LevelFilter};

static LOGGER: logger::Logger = logger::Logger;

#[tokio::main]
async fn main() {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
		.expect("Failed to set logger");

	info!("Starting server...");
	let app: Router = Router::new().merge(routes::create_router());
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
