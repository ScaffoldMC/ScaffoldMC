use crate::core::bin_providers::VersionInfo;

use super::{BinaryListing, BinaryProvider};
use log::error;
use reqwest::Url;
use serde::Deserialize;

static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

// Internal Use

#[derive(Debug, Deserialize)]
struct APIVersionManifest {
	pub latest: APILatestInfo,
	pub versions: Vec<APIVersionInfo>,
}

#[derive(Debug, Deserialize)]
struct APILatestInfo {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Deserialize, Clone)]
struct APIVersionInfo {
	pub id: String,
	#[serde(rename = "type")]
	pub version_type: String,
	pub url: String,
}

pub async fn get_manifest() -> Result<APIVersionManifest, String> {
	let url = format!("{MOJANG_API_URL}/minecraft/version_manifest.json");
	let response = reqwest::get(&url)
		.await
		.map_err(|e| format!("Failed to fetch manifest: {}", e))?;

	if !response.status().is_success() {
		return Err(format!("Received HTTP {}", response.status()));
	}

	let manifest: APIVersionManifest = response
		.json()
		.await
		.map_err(|e| format!("Failed to parse JSON: {}", e))?;

	Ok(manifest)
}

pub async fn get_version_info(version_id: &str) -> Result<APIVersionInfo, String> {
	let manifest = get_manifest().await?;

	let version_info = manifest
		.versions
		.iter()
		.find(|v| v.id == version_id)
		.ok_or_else(|| format!("Version not found: {}", version_id))?;

	Ok(version_info.clone())
}

// Version Listing Implementation

pub struct MojangJavaVersionInfo {
	game_version: String,
	is_prerelease: bool,
}

impl MojangJavaVersionInfo {
	fn new(game_version: String, is_prerelease: bool) -> Self {
		Self {
			game_version,
			is_prerelease,
		}
	}
}

impl VersionInfo for MojangJavaVersionInfo {
	fn game_version(&self) -> &str {
		&self.game_version
	}

	fn is_prerelease(&self) -> bool {
		self.is_prerelease
	}

	fn identifier(&self) -> String {
		self.game_version.clone()
	}
}

pub struct MojangJavaBinaryListing {
	download_url: Url,
	version: MojangJavaVersionInfo,
}

impl MojangJavaBinaryListing {
	pub async fn new(version: MojangJavaVersionInfo, download_url: Url) -> Result<Self, String> {
		Ok(Self {
			version,
			download_url,
		})
	}
}

impl BinaryListing for MojangJavaBinaryListing {
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

impl BinaryProvider for MojangJavaBinaryProvider {
	type Listing = MojangJavaBinaryListing;

	fn new() -> Self {
		Self {}
	}

	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn list_all(&self) -> Result<Vec<Self::Listing>, String> {
		let manifest = get_manifest().await?;

		let mut listings = Vec::new();

		for v in &manifest.versions {
			let version_info =
				MojangJavaVersionInfo::new(v.id.clone(), v.version_type == "snapshot");

			match self.get(version_info).await {
				Ok(listing) => listings.push(listing),
				Err(e) => error!("Failed to create listing for version {}: {}", v.id, e),
			}
		}

		Ok(listings)
	}

	async fn latest(&self) -> Result<Self::Listing, String> {
		let manifest = get_manifest().await?;
		let latest_version = manifest.latest.release;

		let latest_version = MojangJavaVersionInfo::new(latest_version.clone(), false);

		self.get(latest_version).await
	}

	async fn get(&self, version: MojangJavaVersionInfo) -> Result<Self::Listing, String> {
		let version_info = get_version_info(version.game_version()).await?;
		let download_url =
			Url::parse(&version_info.url).map_err(|e| format!("Failed to parse URL: {}", e))?;

		MojangJavaBinaryListing::new(version, download_url).await
	}
}
