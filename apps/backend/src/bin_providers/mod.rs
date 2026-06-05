use reqwest::Url;

use crate::models::hash::HashAlgorithm;

pub mod fabric;
pub mod paper;
pub mod vanilla;

pub struct JavaDependency {
	pub version: u8,
	pub args: Option<Vec<String>>,
}

pub enum DownloadDependency {
	Java(JavaDependency),
}

/// Download information for a binary
pub struct DownloadInfo {
	pub download_url: Url,
	pub file_name: String,
	pub hash: Option<(String, HashAlgorithm)>,
	pub dependencies: Vec<DownloadDependency>,
}
