use serde::Deserialize;

use crate::util::request::get_and_format;

static PAPER_API_URL: &str = "https://fill.papermc.io/v3/projects/paper";

// API Types

#[derive(Debug, Deserialize, Clone)]
pub struct Versions {
	pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Version {
	pub id: String,
	pub builds: Vec<u16>,
	pub java: JavaInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaInfo {
	pub version: JavaVersion,
	pub flags: JavaFlags,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaVersion {
	pub minimum: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFlags {
	pub recommended: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BuildInfo {
	pub id: u16,
	pub downloads: Downloads,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Downloads {
	pub server_default: Download,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Download {
	pub checksums: Checksums,
	pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Checksums {
	pub sha256: String,
}

// API Client

pub struct PaperMetaAPIClient {
	reqwest_client: reqwest::Client,
}

impl PaperMetaAPIClient {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}

	pub async fn get_versions(&self) -> Result<Versions, String> {
		let url = format!("{}/versions", PAPER_API_URL);
		let response: Versions = get_and_format(&self.reqwest_client, &url).await?;

		Ok(response)
	}

	pub async fn get_version(&self, game_version: &str) -> Result<Version, String> {
		let url = format!("{}/versions/{}", PAPER_API_URL, game_version);
		let response: Version = get_and_format(&self.reqwest_client, &url).await?;

		Ok(response)
	}

	pub async fn get_build(&self, game_version: &str, build_id: u16) -> Result<BuildInfo, String> {
		let url = format!(
			"{}/versions/{}/builds/{}",
			PAPER_API_URL, game_version, build_id
		);
		let response: BuildInfo = get_and_format(&self.reqwest_client, &url).await?;

		Ok(response)
	}
}
