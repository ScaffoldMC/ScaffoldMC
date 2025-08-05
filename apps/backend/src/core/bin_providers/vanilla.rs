use super::{BinaryInfo, BinaryProvider};
use crate::{
	core::{
		api_clients::piston_meta::PistonMetaAPIClient,
		bin_providers::BasicVersionProvider,
		version::{vanilla::VanillaVersionInfo, VersionInfo},
	},
	util::hash::HashAlgorithm,
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

// Version Listing Implementation

pub struct VanillaBinaryInfo {
	download_url: Url,
	hash: String,
	version: Arc<dyn VersionInfo>,
	java_version: u8,
}

impl VanillaBinaryInfo {
	pub async fn new(
		version: Arc<dyn VersionInfo>,
		download_url: Url,
		hash: String,
		java_version: u8,
	) -> Result<Self, String> {
		Ok(Self {
			version,
			download_url,
			hash,
			java_version,
		})
	}
}

impl BinaryInfo for VanillaBinaryInfo {
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
		Some((&self.hash, HashAlgorithm::Sha1))
	}

	fn java_version(&self) -> u8 {
		self.java_version
	}
}

// Provider Implementation

pub struct VanillaBinaryProvider {
	api_client: PistonMetaAPIClient,
}

impl VanillaBinaryProvider {
	pub fn new(api_client: PistonMetaAPIClient) -> Self {
		Self { api_client }
	}
}

#[async_trait]
impl BinaryProvider for VanillaBinaryProvider {
	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let manifest = self.api_client.get_manifest().await?;

		let latest_version = if pre_release {
			manifest.latest.snapshot
		} else {
			manifest.latest.release
		};

		let latest_version = VanillaVersionInfo::new(latest_version.clone());

		self.get(Arc::new(latest_version)).await
	}

	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String> {
		// We need to downcast the version to MojangJavaVersionInfo
		let mojang_version = version
			.as_any()
			.downcast_ref::<VanillaVersionInfo>()
			.ok_or("Invalid version type for MojangJavaBinaryProvider")?;

		let version_info = self.api_client.get_version(&mojang_version.game()).await?;

		let download_url = Url::parse(&version_info.downloads.server.url)
			.map_err(|e| format!("Failed to parse URL: {}", e))?;
		let hash = version_info.downloads.server.sha1.clone();

		let java_version = version_info.java_version.major_version;

		let binary_info = VanillaBinaryInfo::new(version, download_url, hash, java_version).await?;
		Ok(Box::new(binary_info))
	}
}

impl BasicVersionProvider for VanillaBinaryProvider {
	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let manifest = self.api_client.get_manifest().await?;

		let mut listings: Vec<Arc<dyn VersionInfo>> = Vec::new();

		for v in &manifest.versions {
			let version_info = VanillaVersionInfo::new(v.id.clone());
			listings.push(Arc::new(version_info));
		}

		Ok(listings)
	}
}
