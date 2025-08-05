use crate::util::request::get_and_format;
use serde::Deserialize;

static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

#[derive(Debug, Deserialize)]
pub struct MojangManifest {
	pub latest: MojangLatestInfo,
	pub versions: Vec<MojangVersionInfo>,
}

#[derive(Debug, Deserialize)]
pub struct MojangLatestInfo {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MojangVersionInfo {
	pub id: String,
	#[serde(rename = "type")]
	pub version_type: String,
	pub url: String,
	pub sha1: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MojangPackageInfo {
	pub id: String,

	#[serde(rename = "javaVersion")]
	pub java_version: MojangJavaVersion,

	pub downloads: MojangDownloads,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MojangJavaVersion {
	#[serde(rename = "majorVersion")]
	pub major_version: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MojangDownloads {
	pub server: MojangDownloadInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MojangDownloadInfo {
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

	pub async fn get_manifest(&self) -> Result<MojangManifest, String> {
		let url = format!("{MOJANG_API_URL}/mc/game/version_manifest_v2.json");
		let manifest: MojangManifest = get_and_format(&self.reqwest_client, &url).await?;
		Ok(manifest)
	}

	pub async fn get_version(&self, version_id: &str) -> Result<MojangPackageInfo, String> {
		let manifest = self.get_manifest().await?;
		let version_info = manifest
			.versions
			.iter()
			.find(|v| v.id == version_id)
			.ok_or_else(|| format!("Version not found: {}", version_id))?;

		let package_info: MojangPackageInfo =
			get_and_format(&self.reqwest_client, &version_info.url).await?;

		Ok(package_info)
	}
}
