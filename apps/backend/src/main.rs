mod logger;
mod routes;

use std::net::SocketAddr;

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

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	axum_server::bind(addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
