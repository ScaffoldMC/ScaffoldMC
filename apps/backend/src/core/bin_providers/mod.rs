use reqwest::Url;

pub mod fabric;
pub mod mojang_java;

pub trait VersionInfo {
	fn game_version(&self) -> &str;
	fn is_prerelease(&self) -> bool;
	fn identifier(&self) -> String;
}

pub trait BinaryListing {
	type Version: VersionInfo;

	fn download_url(&self) -> &Url;
	fn version(&self) -> &Self::Version;
	fn file_name(&self) -> &str;
}

// TODO: Implement caching
pub trait BinaryProvider {
	type Listing: BinaryListing;

	fn new() -> Self;
	fn binary_name(&self) -> &str;

	async fn list_all(&self) -> Result<Vec<Self::Listing>, String>;
	async fn latest(&self) -> Result<Self::Listing, String>;
	async fn get(
		&self,
		version: <Self::Listing as BinaryListing>::Version,
	) -> Result<Self::Listing, String>;
}
