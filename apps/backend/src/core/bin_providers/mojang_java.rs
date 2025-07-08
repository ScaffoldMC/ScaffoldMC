use super::{BinaryListing, BinaryProvider};
use log::error;
use serde::Deserialize;

static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

// Internal Use

#[derive(Debug, Deserialize)]
struct MojangVersionManifest {
	pub latest: LatestInfo,
	pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Deserialize)]
struct LatestInfo {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Deserialize, Clone)]
struct VersionInfo {
	pub id: String,
	#[serde(rename = "type")]
	pub version_type: String,
	pub url: String,
}

pub async fn get_manifest() -> Result<MojangVersionManifest, String> {
	let url = format!("{MOJANG_API_URL}/minecraft/version_manifest.json");
	let response = reqwest::get(&url)
		.await
		.map_err(|e| format!("Failed to fetch manifest: {}", e))?;

	if !response.status().is_success() {
		return Err(format!("Received HTTP {}", response.status()));
	}

	let manifest: MojangVersionManifest = response
		.json()
		.await
		.map_err(|e| format!("Failed to parse JSON: {}", e))?;

	Ok(manifest)
}

pub async fn get_version_info(version_id: &str) -> Result<VersionInfo, String> {
	let manifest = get_manifest().await?;

	let version_info = manifest
		.versions
		.iter()
		.find(|v| v.id == version_id)
		.ok_or_else(|| format!("Version not found: {}", version_id))?;

	Ok(version_info.clone())
}

// Version Listing Implementation

pub struct MojangJavaBinaryListing {
	download_url: String,
	version: String,
}

impl MojangJavaBinaryListing {
	pub async fn new(version: String, download_url: String) -> Result<Self, String> {
		Ok(Self {
			version,
			download_url,
		})
	}
}

impl BinaryListing for MojangJavaBinaryListing {
	fn download_url(&self) -> &str {
		&self.download_url
	}

	fn version(&self) -> &str {
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

	async fn list_all(&self) -> Result<Vec<Self::Listing>, String> {
		let manifest = get_manifest().await?;

		let mut listings = Vec::new();

		for v in &manifest.versions {
			match self.get(&v.id).await {
				Ok(listing) => listings.push(listing),
				Err(e) => error!("Failed to create listing for version {}: {}", v.id, e),
			}
		}

		Ok(listings)
	}

	async fn latest(&self) -> Result<Self::Listing, String> {
		let manifest = get_manifest().await?;
		let latest_version = manifest.latest.release;

		self.get(&latest_version).await
	}

	async fn get(&self, version: &str) -> Result<Self::Listing, String> {
		let version_info = get_version_info(version).await?;
		MojangJavaBinaryListing::new(version.to_string(), version_info.url).await
	}
}
