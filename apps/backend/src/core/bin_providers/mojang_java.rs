use super::{BinaryInfo, BinaryProvider};
use crate::core::version::{mojang_java::MojangJavaVersionInfo, VersionInfo};
use reqwest::Url;
use serde::Deserialize;

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
	}
}

async fn get_manifest() -> Result<api_types::VersionManifest, String> {
	let url = format!("{MOJANG_API_URL}/minecraft/version_manifest.json");
	let response = reqwest::get(&url)
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

async fn get_version_info(version_id: &str) -> Result<api_types::VersionInfo, String> {
	let manifest = get_manifest().await?;

	let version_info = manifest
		.versions
		.iter()
		.find(|v| v.id == version_id)
		.ok_or_else(|| format!("Version not found: {}", version_id))?;

	Ok(version_info.clone())
}

// Version Listing Implementation

pub struct MojangJavaBinaryInfo {
	download_url: Url,
	version: MojangJavaVersionInfo,
}

impl MojangJavaBinaryInfo {
	pub async fn new(version: MojangJavaVersionInfo, download_url: Url) -> Result<Self, String> {
		Ok(Self {
			version,
			download_url,
		})
	}
}

impl BinaryInfo for MojangJavaBinaryInfo {
	type Version = MojangJavaVersionInfo;

	fn download_url(&self) -> &Url {
		&self.download_url
	}

	fn version(&self) -> &MojangJavaVersionInfo {
		&self.version
	}

	fn file_name(&self) -> &str {
		"server.jar"
	}
}

// Provider Implementation

pub struct MojangJavaBinaryProvider;

impl MojangJavaBinaryProvider {
	pub fn new() -> Self {
		Self {}
	}
}

impl BinaryProvider for MojangJavaBinaryProvider {
	type Binary = MojangJavaBinaryInfo;

	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn list_versions(&self) -> Result<Vec<MojangJavaVersionInfo>, String> {
		let manifest = get_manifest().await?;

		let mut listings = Vec::new();

		for v in &manifest.versions {
			let version_info = MojangJavaVersionInfo::new(v.id.clone());

			listings.push(version_info);
		}

		Ok(listings)
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Self::Binary, String> {
		let manifest = get_manifest().await?;
		let latest_version = if pre_release {
			manifest.latest.snapshot
		} else {
			manifest.latest.release
		};

		let latest_version = MojangJavaVersionInfo::new(latest_version.clone());

		self.get(latest_version).await
	}

	async fn get(&self, version: MojangJavaVersionInfo) -> Result<Self::Binary, String> {
		let version_info = get_version_info(version.game_version()).await?;
		let download_url =
			Url::parse(&version_info.url).map_err(|e| format!("Failed to parse URL: {}", e))?;

		MojangJavaBinaryInfo::new(version, download_url).await
	}
}
