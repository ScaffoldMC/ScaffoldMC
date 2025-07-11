use crate::{
	core::{
		bin_providers::{BinaryInfo, BinaryProvider},
		version::{fabric::FabricVersionInfo, VersionInfo},
	},
	services::binary,
};
use reqwest::Url;

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
	version: FabricVersionInfo,
}

impl FabricBinaryInfo {
	pub fn new(version: FabricVersionInfo, download_url: Url) -> Self {
		Self {
			download_url,
			version,
		}
	}
}

impl BinaryInfo for FabricBinaryInfo {
	type Version = FabricVersionInfo;

	fn download_url(&self) -> &Url {
		&self.download_url
	}

	fn version(&self) -> &Self::Version {
		&self.version
	}

	fn file_name(&self) -> &str {
		todo!()
	}
}

pub struct FabricBinaryProvider;

impl FabricBinaryProvider {
	pub fn new() -> Self {
		Self {}
	}
}

impl BinaryProvider for FabricBinaryProvider {
	type Binary = FabricBinaryInfo;

	fn binary_name(&self) -> &str {
		"server.jar"
	}

	// Note: the latest loader and installer versions are used or else the
	// vector will be massive due to the number of versions for each game
	// version. Perhaps a better system for storing these versions could be
	// implemented to allow for more flexibility in the future.

	async fn list_versions(&self) -> Result<Vec<FabricVersionInfo>, String> {
		let url_str = format!("{}/versions/loader/", FABRIC_API_URL);

		let manifest = reqwest::get(&url_str)
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

		let mut versions = Vec::new();

		// Create one version per game version using latest stable loader and installer
		for game_version in &manifest.game {
			let fabric_version = FabricVersionInfo::new(
				game_version.version.clone(),
				latest_loader.version.clone(),
				latest_installer.version.clone(),
				!game_version.stable,
			);
			versions.push(fabric_version);
		}

		Ok(versions)
	}

	async fn get_latest(&self, pre_release: bool) -> Result<FabricBinaryInfo, String> {
		let url_str = format!("{}/versions/loader/", FABRIC_API_URL);

		let manifest = reqwest::get(&url_str)
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
			pre_release,
		);

		let binary_info = self.get(fabric_version).await?;

		Ok(binary_info)
	}

	async fn get(&self, version: FabricVersionInfo) -> Result<Self::Binary, String> {
		let url_str = format!(
			"{}/versions/loader/{}/{}/{}/server/jar",
			FABRIC_API_URL,
			version.game_version(),
			version.fabric_version(),
			version.launcher_version()
		);

		let download_url =
			Url::parse(&url_str).map_err(|e| format!("Failed to parse URL: {}", e))?;

		Ok(FabricBinaryInfo::new(version.into(), download_url))
	}
}
