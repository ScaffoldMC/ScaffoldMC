use crate::{core::game, util::request::get_and_format};
use reqwest::Url;
use serde::Deserialize;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

// API Types

#[derive(Debug, Deserialize, Clone)]
pub struct FabricManifest {
	pub game: Vec<GameVersion>,
	pub loader: Vec<LoaderVersion>,
	pub installer: Vec<InstallerVersion>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameVersion {
	pub version: String,
	pub stable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoaderVersion {
	pub build: i32,
	pub version: String,
	pub stable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstallerVersion {
	pub version: String,
	pub stable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoaderVersionInfo {
	#[serde(rename = "launcherMeta")]
	pub launcher_meta: LauncherMeta,
	pub loader: LoaderVersion,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LauncherMeta {
	pub min_java_version: u8,
}

// API Client

pub struct FabricMetaAPIClient {
	reqwest_client: reqwest::Client,
}

impl FabricMetaAPIClient {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}

	pub async fn get_manifest(&self) -> Result<FabricManifest, String> {
		let url_str = format!("{}/versions", FABRIC_API_URL);
		let manifest: FabricManifest = get_and_format(&self.reqwest_client, &url_str).await?;

		Ok(manifest)
	}

	pub async fn get_versions(&self, game_version: &str) -> Result<Vec<LoaderVersionInfo>, String> {
		let url_str = format!("{}/versions/loader/{}/", FABRIC_API_URL, game_version);
		let versions: Vec<LoaderVersionInfo> =
			get_and_format(&self.reqwest_client, &url_str).await?;

		Ok(versions)
	}

	pub async fn get_version(
		&self,
		game_version: &str,
		fabric_version: &str,
	) -> Result<LoaderVersionInfo, String> {
		let url_str = format!(
			"{}/versions/loader/{}/{}/",
			FABRIC_API_URL, game_version, fabric_version
		);

		let response: LoaderVersionInfo = get_and_format(&self.reqwest_client, &url_str).await?;

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
