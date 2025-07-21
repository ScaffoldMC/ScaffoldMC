use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

use crate::core::version::VersionInfo;

pub mod fabric;
pub mod mojang_java;

pub trait BinaryInfo {
	fn download_url(&self) -> &Url;
	fn version(&self) -> Arc<dyn VersionInfo>;
	fn file_name(&self) -> &str;
}

// TODO: Implement caching
#[async_trait]
pub trait BinaryProvider: Send + Sync {
	fn binary_name(&self) -> &str;

	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String>;
	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String>;
	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String>;
}
