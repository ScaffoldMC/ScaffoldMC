use crate::util::request::get_and_format;
use reqwest::Url;
use serde::Deserialize;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

// API Types

#[derive(Debug, Deserialize, Clone)]
pub struct FabricVersionsManifest {
	pub game: Vec<FabricGameVersion>,
	pub loader: Vec<FabricLoaderVersion>,
	pub installer: Vec<FabricInstallerVersion>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FabricGameVersion {
	pub version: String,
	pub stable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FabricLoaderVersion {
	pub build: i32,
	pub version: String,
	pub stable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FabricInstallerVersion {
	pub version: String,
	pub stable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FabricLoaderInfo {
	pub loader: FabricLoaderVersion,
}

// API Client

pub struct FabricMetaAPIClient {
	reqwest_client: reqwest::Client,
}

impl FabricMetaAPIClient {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}

	pub async fn get_manifest(&self) -> Result<FabricVersionsManifest, String> {
		let url_str = format!("{}/versions", FABRIC_API_URL);
		let manifest: FabricVersionsManifest =
			get_and_format(&self.reqwest_client, &url_str).await?;

		Ok(manifest)
	}

	pub async fn get_versions(&self, game_version: &str) -> Result<Vec<FabricLoaderInfo>, String> {
		let url_str = format!("{}/versions/loader/{}/", FABRIC_API_URL, game_version);
		let versions: Vec<FabricLoaderInfo> =
			get_and_format(&self.reqwest_client, &url_str).await?;

		Ok(versions)
	}

	pub async fn get_version(
		&self,
		game_version: &str,
		fabric_version: &str,
	) -> Result<FabricLoaderInfo, String> {
		let url_str = format!(
			"{}/versions/loader/{}/{}/",
			FABRIC_API_URL, game_version, fabric_version
		);

		let response: FabricLoaderInfo = get_and_format(&self.reqwest_client, &url_str).await?;

		Ok(response)
	}

	pub async fn get_download_url(
		&self,
		game_version: &str,
		fabric_version: &str,
		installer_version: &str,
	) -> Result<Url, String> {
		let url_str = format!(
			"{}/versions/loader/{}/{}/{}/server/jar",
			FABRIC_API_URL, game_version, fabric_version, installer_version
		);

		let download_url =
			Url::parse(&url_str).map_err(|e| format!("Failed to parse URL: {}", e))?;

		Ok(download_url)
	}
}
