use serde::Deserialize;
use thiserror::Error;

static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

#[derive(Debug, Deserialize)]
pub struct MojangVersionManifest {
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

#[derive(Debug, Error)]
pub enum MojangAPIError {
	#[error("Failed to fetch data from Mojang API: {0}")]
	FetchError(String),
	#[error("Failed to parse Mojang API response: {0}")]
	ParseError(String),
	#[error("Version not found: {0}")]
	VersionNotFound(String),
}

pub async fn get_manifest() -> Result<MojangVersionManifest, MojangAPIError> {
	let url = format!("{MOJANG_API_URL}/minecraft/version_manifest.json");
	let response = reqwest::get(&url)
		.await
		.map_err(|e| MojangAPIError::FetchError(e.to_string()))?;

	if !response.status().is_success() {
		return Err(MojangAPIError::FetchError(format!(
			"Received HTTP {}",
			response.status()
		)));
	}

	let manifest: MojangVersionManifest = response
		.json()
		.await
		.map_err(|e| MojangAPIError::FetchError(e.to_string()))?;

	Ok(manifest)
}

pub async fn get_version_info(version_id: &str) -> Result<VersionInfo, MojangAPIError> {
	let manifest = get_manifest().await?;

	let version_info = manifest
		.versions
		.iter()
		.find(|v| v.id == version_id)
		.ok_or_else(|| MojangAPIError::VersionNotFound(version_id.to_string()))?;

	Ok(version_info.clone())
}
