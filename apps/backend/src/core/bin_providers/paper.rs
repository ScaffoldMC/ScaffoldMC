use crate::{
	core::{
		bin_providers::{BinaryInfo, BinaryProvider},
		version::{paper::PaperVersionInfo, VersionInfo},
	},
	util::hash::HashAlgorithm,
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

mod api_types {
	use serde::Deserialize;

	#[derive(Debug, Deserialize, Clone)]
	pub struct Versions {
		pub versions: Vec<Version>,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct Version {
		pub id: String,
		pub builds: Vec<u16>,
		pub java: JavaInfo,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct JavaInfo {
		pub version: JavaVersion,
		pub flags: JavaFlags,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct JavaVersion {
		pub minimum: u8,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct JavaFlags {
		pub recommended: Vec<String>,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct BuildInfo {
		pub id: u16,
		pub downloads: Downloads,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct Downloads {
		pub server_default: Download,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct Download {
		pub checksums: Checksums,
		pub url: String,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct Checksums {
		pub sha256: String,
	}
}

static PAPER_API_URL: &str = "https://fill.papermc.io/v3/projects/paper";

pub struct PaperBinaryInfo {
	download_url: Url,
	hash: String,
	version: Arc<dyn VersionInfo>,
	java_version: u8,
	java_args: Vec<String>,
}

impl PaperBinaryInfo {
	pub fn new(
		version: Arc<dyn VersionInfo>,
		download_url: Url,
		hash: String,
		java_version: u8,
		java_args: Vec<String>,
	) -> Self {
		Self {
			download_url,
			version,
			hash,
			java_version,
			java_args,
		}
	}
}

impl BinaryInfo for PaperBinaryInfo {
	fn download_url(&self) -> &Url {
		&self.download_url
	}

	fn version(&self) -> Arc<dyn VersionInfo> {
		Arc::clone(&self.version)
	}

	fn file_name(&self) -> &str {
		"server.jar"
	}

	fn hash(&self) -> Option<(&str, HashAlgorithm)> {
		Some((&self.hash, HashAlgorithm::Sha256))
	}

	fn java_version(&self) -> u8 {
		self.java_version
	}

	fn java_rec_args(&self) -> Vec<String> {
		self.java_args.clone()
	}
}

pub struct PaperBinaryProvider {
	reqwest_client: reqwest::Client,
}

impl PaperBinaryProvider {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}
}

#[async_trait]
impl BinaryProvider for PaperBinaryProvider {
	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let url = format!("{}/versions", PAPER_API_URL);

		let response = self
			.reqwest_client
			.get(&url)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch versions: {}", e))?
			.json::<api_types::Versions>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let mut versions: Vec<Arc<dyn VersionInfo>> = Vec::new();

		for version in response.versions {
			for build in version.builds {
				let version_info = PaperVersionInfo::new(version.id.clone(), build);
				versions.push(Arc::new(version_info));
			}
		}

		Ok(versions)
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let url = format!("{}/versions", PAPER_API_URL);

		let response = self
			.reqwest_client
			.get(&url)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch versions: {}", e))?
			.json::<api_types::Versions>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		// TODO: Handle pre-release logic
		let latest_version = response.versions.first().ok_or("No versions found")?;

		let version_info =
			PaperVersionInfo::new(latest_version.id.clone(), latest_version.builds[0]);

		self.get(Arc::new(version_info)).await
	}

	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String> {
		let paper_version = version
			.as_any()
			.downcast_ref::<PaperVersionInfo>()
			.ok_or("Invalid version type for PaperBinaryProvider")?;

		let url = format!("{}/versions/{}", PAPER_API_URL, paper_version.game(),);

		let response = self
			.reqwest_client
			.get(&url)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch version info: {}", e))?
			.json::<api_types::Version>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let java_version = response.java.version.minimum;
		let java_args = response.java.flags.recommended;

		let url = format!(
			"{}/versions/{}/builds/{}",
			PAPER_API_URL,
			paper_version.game(),
			paper_version.paper_build()
		);

		let response = self
			.reqwest_client
			.get(&url)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch version info: {}", e))?
			.json::<api_types::BuildInfo>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let download_url = Url::parse(&response.downloads.server_default.url)
			.map_err(|e| format!("Failed to parse download URL: {}", e))?;

		let hash = response.downloads.server_default.checksums.sha256;

		let binary_info = PaperBinaryInfo::new(
			Arc::clone(&version),
			download_url,
			hash,
			java_version,
			java_args,
		);

		Ok(Box::new(binary_info))
	}
}
