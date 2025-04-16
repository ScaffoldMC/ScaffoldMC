mod routes;

use axum::Router;

#[tokio::main]
async fn main() {
	let app: Router = Router::new().merge(routes::create_router());
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
