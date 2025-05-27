use log::info;

pub mod user;

pub struct Database {
	pub pool: sqlx::SqlitePool,
}

impl Database {
	pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
		let options = sqlx::sqlite::SqliteConnectOptions::new()
			.filename(db_path)
			.create_if_missing(true);

		info!("Connecting to database at {}", db_path);
		let pool = sqlx::SqlitePool::connect_with(options)
			.await
			.expect("Failed to connect to database");

		info!("Running database migrations");
		sqlx::migrate!("./migrations")
			.run(&pool)
			.await
			.expect("Failed to run migrations");

		Ok(Self { pool })
	}
}
