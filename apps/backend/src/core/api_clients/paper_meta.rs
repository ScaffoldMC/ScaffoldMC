use serde::Deserialize;

use crate::util::request::get_and_format;

static PAPER_API_URL: &str = "https://fill.papermc.io/v3/projects/paper";

// API Types

#[derive(Debug, Deserialize, Clone)]
pub struct PaperVersions {
	pub versions: Vec<PaperVersionAndBuilds>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperVersion {
	pub id: String,
	pub java: PaperJavaInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperVersionAndBuilds {
	pub version: PaperVersion,
	pub builds: Vec<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperJavaInfo {
	pub version: PaperJavaVersion,
	pub flags: PaperJavaFlags,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperJavaVersion {
	pub minimum: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperJavaFlags {
	pub recommended: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperBuildInfo {
	pub id: u16,
	pub downloads: PaperDownloads,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperDownloads {
	#[serde(rename = "server:default")]
	pub server_default: PaperDownload,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperDownload {
	pub checksums: PaperChecksums,
	pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaperChecksums {
	pub sha256: String,
}

// API Client

pub struct PaperDownloadsAPIClient {
	reqwest_client: reqwest::Client,
}

impl PaperDownloadsAPIClient {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}

	pub async fn get_versions(&self) -> Result<PaperVersions, String> {
		let url = format!("{}/versions", PAPER_API_URL);
		let response: PaperVersions = get_and_format(&self.reqwest_client, &url).await?;

		Ok(response)
	}

	pub async fn get_version(&self, game_version: &str) -> Result<PaperVersionAndBuilds, String> {
		let url = format!("{}/versions/{}", PAPER_API_URL, game_version);
		let response: PaperVersionAndBuilds = get_and_format(&self.reqwest_client, &url).await?;

		Ok(response)
	}

	pub async fn get_build(
		&self,
		game_version: &str,
		build_id: u16,
	) -> Result<PaperBuildInfo, String> {
		let url = format!(
			"{}/versions/{}/builds/{}",
			PAPER_API_URL, game_version, build_id
		);
		let response: PaperBuildInfo = get_and_format(&self.reqwest_client, &url).await?;

		Ok(response)
	}
}
