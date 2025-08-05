static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

// Internal Use

use serde::Deserialize;

use crate::util::request::get_and_format;

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

#[derive(Debug, Deserialize, Clone)]
pub struct PackageInfo {
	pub id: String,

	#[serde(rename = "javaVersion")]
	pub java_version: JavaVersion,

	pub downloads: Downloads,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaVersion {
	#[serde(rename = "majorVersion")]
	pub major_version: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Downloads {
	pub server: DownloadInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DownloadInfo {
	pub url: String,
	pub sha1: String,
}

pub struct PistonMetaAPIClient {
	reqwest_client: reqwest::Client,
}

impl PistonMetaAPIClient {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}

	pub async fn get_manifest(&self) -> Result<VersionManifest, String> {
		let url = format!("{MOJANG_API_URL}/mc/game/version_manifest_v2.json");
		let manifest: VersionManifest = get_and_format(&self.reqwest_client, &url).await?;
		Ok(manifest)
	}

	pub async fn get_version(&self, version_id: &str) -> Result<PackageInfo, String> {
		let manifest = self.get_manifest().await?;
		let version_info = manifest
			.versions
			.iter()
			.find(|v| v.id == version_id)
			.ok_or_else(|| format!("Version not found: {}", version_id))?;

		let package_info: PackageInfo =
			get_and_format(&self.reqwest_client, &version_info.url).await?;

		Ok(package_info)
	}
}
