use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

use crate::{core::version::VersionInfo, util::hash::HashAlgorithm};

pub mod fabric;
pub mod paper;
pub mod vanilla;

pub trait BinaryInfo: Send + Sync {
	/// Get the download URL for this binary
	fn download_url(&self) -> &Url;

	/// Get the version information of this binary
	fn version(&self) -> Arc<dyn VersionInfo>;

	/// Get the file name of the binary, e.g. "server.jar"
	fn file_name(&self) -> &str;

	/// Get the hash of the binary, if available
	fn hash(&self) -> Option<(&str, HashAlgorithm)> {
		// Default to none in case the provider does not provide a hash
		None
	}

	/// Get the Java version required by this binary
	fn java_version(&self) -> u8;

	/// Get the Java arguments required by this binary
	fn java_rec_args(&self) -> Vec<String> {
		// Default to empty args in case the provider does not provide any
		Vec::new()
	}
}

#[async_trait]
pub trait BinaryProvider: Send + Sync {
	fn binary_name(&self) -> &str;

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String>;
	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String>;
}

/// Version provider for vanilla-style versions with low version complexity
/// (i.e. game version, build number)
pub trait BasicVersionProvider {
	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String>;
}

/// Version provider trait for binaries with more complex versioning schemes
/// (i.e. game version, loader version, installer version, with cross-compatibility)
pub trait AdvancedVersionProvider {
	async fn list_game_versions(&self) -> Result<Vec<String>, String>;
	async fn list_loader_versions(
		&self,
		game_version: &str,
	) -> Result<Vec<Arc<dyn VersionInfo>>, String>;
}
