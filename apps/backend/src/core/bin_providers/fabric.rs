use crate::core::{
	bin_providers::{AdvancedVersionProvider, BinaryInfo, BinaryProvider},
	version::{fabric::FabricVersionInfo, VersionInfo},
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

mod api_types {
	use serde::Deserialize;

	#[derive(Debug, Deserialize, Clone)]
	pub struct Manifest {
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
}

pub struct FabricBinaryInfo {
	download_url: Url,
	version: Arc<dyn VersionInfo>,
	java_version: u8,
}

impl FabricBinaryInfo {
	pub fn new(version: Arc<dyn VersionInfo>, download_url: Url, java_version: u8) -> Self {
		Self {
			download_url,
			version,
			java_version,
		}
	}
}

impl BinaryInfo for FabricBinaryInfo {
	fn download_url(&self) -> &Url {
		&self.download_url
	}

	fn version(&self) -> Arc<dyn VersionInfo> {
		Arc::clone(&self.version)
	}

	fn file_name(&self) -> &str {
		"server.jar"
	}

	fn java_version(&self) -> u8 {
		self.java_version
	}
}

pub struct FabricBinaryProvider {
	reqwest_client: reqwest::Client,
}

impl FabricBinaryProvider {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		Self { reqwest_client }
	}
}

#[async_trait]
impl BinaryProvider for FabricBinaryProvider {
	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let url_str = format!("{}/versions", FABRIC_API_URL);

		let manifest = self
			.reqwest_client
			.get(&url_str)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch versions: {}", e))?
			.json::<api_types::Manifest>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let latest_loader = manifest
			.loader
			.iter()
			.find(|v| v.stable)
			.ok_or("No stable loader versions found")?;

		let latest_installer = manifest
			.installer
			.iter()
			.find(|v| v.stable)
			.ok_or("No stable installer versions found")?;

		let latest_game = manifest
			.game
			.iter()
			.find(|v| v.stable == !pre_release)
			.ok_or("No suitable game versions found")?;

		let fabric_version = FabricVersionInfo::new(
			latest_game.version.clone(),
			latest_loader.version.clone(),
			latest_installer.version.clone(),
		);

		self.get(Arc::new(fabric_version)).await
	}

	async fn get(&self, version: Arc<dyn VersionInfo>) -> Result<Box<dyn BinaryInfo>, String> {
		// We need to downcast the version to FabricVersionInfo
		let fabric_version = version
			.as_any()
			.downcast_ref::<FabricVersionInfo>()
			.ok_or("Invalid version type for FabricBinaryProvider")?;

		let url_str = format!(
			"{}/versions/loader/{}/{}/",
			FABRIC_API_URL,
			fabric_version.game(),
			fabric_version.fabric()
		);

		let response = self
			.reqwest_client
			.get(&url_str)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch version info: {}", e))?
			.json::<api_types::LoaderVersionInfo>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let java_version = response.launcher_meta.min_java_version;

		let url_str = format!(
			"{}/versions/loader/{}/{}/{}/server/jar",
			FABRIC_API_URL,
			fabric_version.game(),
			fabric_version.fabric(),
			fabric_version.launcher()
		);

		let download_url =
			Url::parse(&url_str).map_err(|e| format!("Failed to parse URL: {}", e))?;

		let binary_info = FabricBinaryInfo::new(version, download_url, java_version);

		Ok(Box::new(binary_info))
	}
}

impl AdvancedVersionProvider for FabricBinaryProvider {
	async fn list_game_versions(&self) -> Result<Vec<String>, String> {
		let url_str = format!("{}/versions", FABRIC_API_URL);

		let manifest = self
			.reqwest_client
			.get(&url_str)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch versions: {}", e))?
			.json::<api_types::Manifest>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let versions: Vec<String> = manifest.game.iter().map(|v| v.version.clone()).collect();

		Ok(versions)
	}

	async fn list_loader_versions(
		&self,
		game_version: &str,
	) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let url_str = format!("{}/versions/", FABRIC_API_URL);

		let manifest = self
			.reqwest_client
			.get(&url_str)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch versions: {}", e))?
			.json::<api_types::Manifest>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let latest_installer: String = manifest
			.installer
			.iter()
			.filter(|v| v.stable)
			.nth(0)
			.ok_or("No stable installer versions found")?
			.version
			.clone();

		let url_str = format!("{}/versions/loader/{}/", FABRIC_API_URL, game_version);

		let loaders = self
			.reqwest_client
			.get(&url_str)
			.send()
			.await
			.map_err(|e| format!("Failed to fetch loader versions: {}", e))?
			.json::<Vec<api_types::LoaderVersionInfo>>()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		let versions: Vec<Arc<dyn VersionInfo>> = loaders
			.iter()
			.map(|loader| {
				let fabric_version = Arc::new(FabricVersionInfo::new(
					game_version.to_string(),
					loader.loader.version.clone(),
					latest_installer.clone(),
				));
				fabric_version as Arc<dyn VersionInfo>
			})
			.collect();

		Ok(versions)
	}
}
