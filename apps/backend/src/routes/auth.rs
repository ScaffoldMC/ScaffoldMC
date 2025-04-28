use axum::{routing::get, Router};

pub fn create_router() -> Router {
	Router::new()
		.route("/login", get(login))
		.route("/logout", get(logout))
}

async fn login() {
	todo!("Login logic");
}

async fn logout() {
	todo!("Logout logic");
}
