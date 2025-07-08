pub mod fabric;
pub mod mojang_java;

pub trait BinaryListing {
	fn download_url(&self) -> &str;
	fn version(&self) -> &str;
	fn file_name(&self) -> &str;
}

// TODO: Implement caching
pub trait BinaryProvider<T: BinaryListing> {
	fn new() -> Self;

	async fn list_all(&self) -> Result<Vec<T>, String>;
	async fn latest(&self) -> Result<T, String>;
}
