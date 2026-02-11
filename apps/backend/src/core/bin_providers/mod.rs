use reqwest::Url;

use crate::util::hash::HashAlgorithm;

pub mod fabric;
pub mod paper;
pub mod vanilla;

pub enum DownloadInfo {
	MinecraftJava(MCJEDownloadInfo),
}

pub struct MCJEDownloadInfo {
	download_url: Url,
	file_name: String,
	hash: Option<(String, HashAlgorithm)>,
	java_version: u8,
	java_args: Vec<String>,
}

impl MCJEDownloadInfo {
	/// Get the download URL of the binary
	pub fn download_url(&self) -> &Url {
		&self.download_url
	}

	/// Get the file name of the binary
	pub fn file_name(&self) -> &str {
		&self.file_name
	}

	/// Get the hash and its algorithm, if available
	pub fn hash(&self) -> Option<(String, HashAlgorithm)> {
		match self.hash {
			Some((ref h, ref alg)) => Some((h.clone(), *alg)),
			None => None,
		}
	}

	/// Get the Java version required by this binary
	pub fn java_version(&self) -> u8 {
		self.java_version
	}

	/// Get the Java arguments required by this binary
	pub fn java_rec_args(&self) -> Vec<String> {
		self.java_args.clone()
	}
}
