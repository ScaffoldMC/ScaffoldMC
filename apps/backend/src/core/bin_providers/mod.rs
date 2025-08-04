use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

use crate::{core::version::VersionInfo, util::hash::HashAlgorithm};

pub mod fabric;
pub mod mojang_java;
pub mod paper;

pub trait BinaryInfo {
	fn download_url(&self) -> &Url;
	fn version(&self) -> Arc<dyn VersionInfo>;
	fn file_name(&self) -> &str;
	fn hash(&self) -> Option<(&str, HashAlgorithm)> {
		// Default to none in case the provider does not provide a hash
		None
	}
}

pub trait JavaBinaryInfo: BinaryInfo {
	fn java_version(&self) -> u8;
	fn recommended_args(&self) -> Vec<String> {
		// Default to empty args in case the provider does not provide any
		Vec::new()
	}
}

// TODO: Implement caching
#[async_trait]
pub trait BinaryProvider: Send + Sync {
	fn binary_name(&self) -> &str;

	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String>;
	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String>;
	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String>;
}
