pub mod user;

pub struct Database {
	pub pool: sqlx::SqlitePool,
}

impl Database {
	pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
		let options = sqlx::sqlite::SqliteConnectOptions::new()
			.filename(db_path)
			.create_if_missing(true);

		let pool = sqlx::SqlitePool::connect_with(options).await?;
		Ok(Self { pool })
	}

	pub async fn migrate(&self) -> Result<(), sqlx::Error> {
		sqlx::migrate!("./migrations").run(&self.pool).await?;
		Ok(())
	}
}
