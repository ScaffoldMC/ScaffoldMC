use super::{BinaryInfo, BinaryProvider};
use crate::{
	core::version::{mojang_java::MojangJavaVersionInfo, VersionInfo},
	util::hash::HashAlgorithm,
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

// Internal Use

mod api_types {
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	pub struct VersionManifest {
		pub latest: LatestInfo,
		pub versions: Vec<VersionInfo>,
	}

	#[derive(Debug, Deserialize)]
	pub struct LatestInfo {
		pub release: String,
		pub snapshot: String,
	}

	#[derive(Debug, Deserialize, Clone)]
	pub struct VersionInfo {
		pub id: String,
		#[serde(rename = "type")]
		pub version_type: String,
		pub url: String,
		pub sha1: String,
	}
}

// Version Listing Implementation

pub struct MojangJavaBinaryInfo {
	download_url: Url,
	hash: String,
	version: Arc<dyn VersionInfo>,
}

impl MojangJavaBinaryInfo {
	pub async fn new(
		version: Arc<dyn VersionInfo>,
		download_url: Url,
		hash: String,
	) -> Result<Self, String> {
		Ok(Self {
			version,
			download_url,
			hash,
		})
	}
}

impl BinaryInfo for MojangJavaBinaryInfo {
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
}

// Provider Implementation

pub struct MojangJavaBinaryProvider {
	reqwest_client: reqwest::Client,
}

impl MojangJavaBinaryProvider {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}

	async fn get_manifest(&self) -> Result<api_types::VersionManifest, String> {
		let url = format!("{MOJANG_API_URL}/mc/game/version_manifest_v2.json");
		let response = self
			.reqwest_client
			.get(&url)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch manifest: {}", e))?;

		if !response.status().is_success() {
			return Err(format!("Received HTTP {}", response.status()));
		}

		let manifest: api_types::VersionManifest = response
			.json()
			.await
			.map_err(|e| format!("Failed to parse JSON: {}", e))?;

		Ok(manifest)
	}

	async fn get_version_info(&self, version_id: &str) -> Result<api_types::VersionInfo, String> {
		let manifest = self.get_manifest().await?;

		let version_info = manifest
			.versions
			.iter()
			.find(|v| v.id == version_id)
			.ok_or_else(|| format!("Version not found: {}", version_id))?;

		Ok(version_info.clone())
	}
}

#[async_trait]
impl BinaryProvider for MojangJavaBinaryProvider {
	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let manifest = self.get_manifest().await?;

		let mut listings: Vec<Arc<dyn VersionInfo>> = Vec::new();

		for v in &manifest.versions {
			let version_info = MojangJavaVersionInfo::new(v.id.clone());
			listings.push(Arc::new(version_info));
		}

		Ok(listings)
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let manifest = self.get_manifest().await?;
		let latest_version = if pre_release {
			manifest.latest.snapshot
		} else {
			manifest.latest.release
		};

		let latest_version = MojangJavaVersionInfo::new(latest_version.clone());

		self.get(Arc::new(latest_version)).await
	}

	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String> {
		// We need to downcast the version to MojangJavaVersionInfo
		let mojang_version = version
			.as_any()
			.downcast_ref::<MojangJavaVersionInfo>()
			.ok_or("Invalid version type for MojangJavaBinaryProvider")?;

		let version_info = self.get_version_info(mojang_version.game()).await?;
		let download_url =
			Url::parse(&version_info.url).map_err(|e| format!("Failed to parse URL: {}", e))?;
		let hash = version_info.sha1.clone();

		let binary_info = MojangJavaBinaryInfo::new(version, download_url, hash).await?;
		Ok(Box::new(binary_info))
	}
}
