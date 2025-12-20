use reqwest::Url;

use crate::util::hash::HashAlgorithm;

pub mod fabric;
pub mod paper;
pub mod vanilla;

pub trait DownloadInfo: Send + Sync {
	/// Get the download URL for this binary
	fn download_url(&self) -> &Url;

	/// Get the file name of the binary, e.g. "server.jar"
	fn file_name(&self) -> &str;

	/// Get the hash of the binary, if available
	fn hash(&self) -> Option<(String, HashAlgorithm)> {
		// Default to none in case the provider does not provide a hash
		None
	}
}

pub struct JavaDownloadInfo {
	download_url: Url,
	file_name: String,
	hash: Option<(String, HashAlgorithm)>,
	java_version: u8,
	java_args: Vec<String>,
}

impl DownloadInfo for JavaDownloadInfo {
	fn download_url(&self) -> &Url {
		&self.download_url
	}

	fn file_name(&self) -> &str {
		&self.file_name
	}

	fn hash(&self) -> Option<(String, HashAlgorithm)> {
		match self.hash {
			Some((ref h, ref alg)) => Some((h.clone(), alg.clone())),
			None => None,
		}
	}
}

impl JavaDownloadInfo {
	/// Get the Java version required by this binary
	fn java_version(&self) -> u8 {
		self.java_version
	}

	/// Get the Java arguments required by this binary
	fn java_rec_args(&self) -> Vec<String> {
		self.java_args.clone()
	}
}
