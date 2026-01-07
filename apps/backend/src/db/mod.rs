use std::path::PathBuf;

use tracing::instrument;

pub mod refresh_token;
pub mod user;

#[derive(Clone)]
pub struct Database {
	pub pool: sqlx::SqlitePool,
}

impl Database {
	#[instrument(name = "Database::new", skip(db_path))]
	pub async fn new(db_path: &PathBuf) -> Result<Self, sqlx::Error> {
		let options = sqlx::sqlite::SqliteConnectOptions::new()
			.filename(db_path)
			.create_if_missing(true);

		tracing::debug!("Connecting to database at {}", db_path.display());
		let pool = sqlx::SqlitePool::connect_with(options)
			.await
			.expect("Failed to connect to database");

		tracing::debug!("Running database migrations");
		sqlx::migrate!("./migrations")
			.run(&pool)
			.await
			.expect("Failed to run migrations");

		Ok(Self { pool })
	}
}
