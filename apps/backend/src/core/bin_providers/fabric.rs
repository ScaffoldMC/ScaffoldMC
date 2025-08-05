use crate::{
	core::{
		api_clients::fabric::FabricMetaAPIClient,
		bin_providers::{AdvancedVersionProvider, BinaryInfo, BinaryProvider},
		version::{fabric::FabricVersionInfo, VersionInfo},
	},
	util::request::get_and_format,
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

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
	api_client: FabricMetaAPIClient,
}

impl FabricBinaryProvider {
	pub fn new(api_client: FabricMetaAPIClient) -> Self {
		Self { api_client }
	}
}

#[async_trait]
impl BinaryProvider for FabricBinaryProvider {
	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Box<dyn BinaryInfo>, String> {
		let manifest = self.api_client.get_manifest().await?;

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

		let version_info = self
			.api_client
			.get_version(fabric_version.game(), fabric_version.fabric())
			.await?;

		let java_version = version_info.launcher_meta.min_java_version;

		let download_url = self
			.api_client
			.get_download_url(
				fabric_version.game(),
				fabric_version.fabric(),
				fabric_version.launcher(),
			)
			.await?;

		let binary_info = FabricBinaryInfo::new(version, download_url, java_version);

		Ok(Box::new(binary_info))
	}
}

impl AdvancedVersionProvider for FabricBinaryProvider {
	async fn list_game_versions(&self) -> Result<Vec<String>, String> {
		let manifest = self.api_client.get_manifest().await?;
		let versions: Vec<String> = manifest.game.iter().map(|v| v.version.clone()).collect();

		Ok(versions)
	}

	async fn list_loader_versions(
		&self,
		game_version: &str,
	) -> Result<Vec<Arc<dyn VersionInfo>>, String> {
		let manifest = self.api_client.get_manifest().await?;

		let latest_installer: String = manifest
			.installer
			.iter()
			.filter(|v| v.stable)
			.nth(0)
			.ok_or("No stable installer versions found")?
			.version
			.clone();

		let loaders = self
			.api_client
			.get_versions(game_version)
			.await
			.map_err(|e| format!("Failed to get loader versions: {}", e))?;

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
