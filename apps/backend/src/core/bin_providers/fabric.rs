use crate::core::{
	bin_providers::{BinaryInfo, BinaryProvider},
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
}

pub struct FabricBinaryInfo {
	download_url: Url,
	version: Arc<dyn VersionInfo>,
}

impl FabricBinaryInfo {
	pub fn new(version: Arc<dyn VersionInfo>, download_url: Url) -> Self {
		Self {
			download_url,
			version,
		}
	}
}

// TODO: Implement JavaBinaryInfo

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

	// Note: the latest loader and installer versions are used or else the
	// vector will be massive due to the number of versions for each game
	// version. Perhaps a better system for storing these versions could be
	// implemented to allow for more flexibility in the future.

	async fn list_versions(&self) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let url_str = format!("{}/versions/loader/", FABRIC_API_URL);

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

		let mut versions: Vec<Arc<dyn VersionInfo>> = Vec::new();

		// Create one version per game version using latest stable loader and installer
		for game_version in &manifest.game {
			let fabric_version = FabricVersionInfo::new(
				game_version.version.clone(),
				latest_loader.version.clone(),
				latest_installer.version.clone(),
			);
			versions.push(Arc::new(fabric_version));
		}

		Ok(versions)
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let url_str = format!("{}/versions/loader/", FABRIC_API_URL);

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
			"{}/versions/loader/{}/{}/{}/server/jar",
			FABRIC_API_URL,
			fabric_version.game(),
			fabric_version.fabric(),
			fabric_version.launcher()
		);

		let download_url =
			Url::parse(&url_str).map_err(|e| format!("Failed to parse URL: {}", e))?;

		let binary_info = FabricBinaryInfo::new(version, download_url);
		Ok(Box::new(binary_info))
	}
}
