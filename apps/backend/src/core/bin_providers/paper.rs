use crate::{
	core::{
		api_clients::paper::PaperMetaAPIClient,
		bin_providers::{BasicVersionProvider, BinaryInfo, BinaryProvider},
		version::{paper::PaperVersionInfo, VersionInfo},
	},
	util::hash::HashAlgorithm,
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

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
	api_client: PaperMetaAPIClient,
}

impl PaperBinaryProvider {
	pub fn new(api_client: PaperMetaAPIClient) -> Self {
		Self { api_client }
	}
}

#[async_trait]
impl BinaryProvider for PaperBinaryProvider {
	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let versions = self.api_client.get_versions().await?;

		// TODO: Handle pre-release logic
		let latest_version = versions.versions.first().ok_or("No versions found")?;

		let version_info =
			PaperVersionInfo::new(latest_version.id.clone(), latest_version.builds[0]);

		self.get(Arc::new(version_info)).await
	}

	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String> {
		let paper_version = version
			.as_any()
			.downcast_ref::<PaperVersionInfo>()
			.ok_or("Invalid version type for PaperBinaryProvider")?;

		let response = self.api_client.get_version(&paper_version.game()).await?;

		let java_version = response.java.version.minimum;
		let java_args = response.java.flags.recommended;

		let build = self
			.api_client
			.get_build(&paper_version.game(), paper_version.paper_build())
			.await?;

		let download_url = Url::parse(&build.downloads.server_default.url)
			.map_err(|e| format!("Failed to parse download URL: {}", e))?;

		let hash = build.downloads.server_default.checksums.sha256;

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

impl BasicVersionProvider for PaperBinaryProvider {
	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let response = self.api_client.get_versions().await?;
		let mut versions: Vec<Arc<dyn VersionInfo>> = Vec::new();

		for version in response.versions {
			for build in version.builds {
				let version_info = PaperVersionInfo::new(version.id.clone(), build);
				versions.push(Arc::new(version_info));
			}
		}

		Ok(versions)
	}
}
