use reqwest::Url;

pub mod fabric;
pub mod mojang_java;

pub trait VersionInfo {
	fn game_version(&self) -> &str;
	fn is_prerelease(&self) -> bool;
	fn identifier(&self) -> String;
}

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
