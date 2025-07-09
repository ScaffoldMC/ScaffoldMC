use reqwest::Url;

use crate::core::version::VersionInfo;

pub mod fabric;
pub mod mojang_java;

pub trait BinaryInfo {
	type Version: VersionInfo;

	fn download_url(&self) -> &Url;
	fn version(&self) -> &Self::Version;
	fn file_name(&self) -> &str;
}

// TODO: Implement caching
pub trait BinaryProvider {
	type Binary: BinaryInfo;

	fn new() -> Self;
	fn binary_name(&self) -> &str;

	async fn list_versions(&self) -> Result<Vec<<Self::Binary as BinaryInfo>::Version>, String>;
	async fn get_latest(&self, pre_release: bool) -> Result<Self::Binary, String>;
	async fn get(
		&self,
		version: <Self::Binary as BinaryInfo>::Version,
	) -> Result<Self::Binary, String>;
}
