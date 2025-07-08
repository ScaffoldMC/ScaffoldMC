pub mod fabric;
pub mod mojang_java;

pub trait BinaryListing {
	fn download_url(&self) -> &str;
	fn version(&self) -> &str;
	fn file_name(&self) -> &str;
}

// TODO: Implement caching
pub trait BinaryProvider {
	type Listing: BinaryListing;

	fn new() -> Self;
	fn binary_name(&self) -> &str;

	async fn list_all(&self) -> Result<Vec<Self::Listing>, String>;
	async fn latest(&self) -> Result<Self::Listing, String>;
	async fn get(&self, version: &str) -> Result<Self::Listing, String>;
}
