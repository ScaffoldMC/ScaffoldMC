use crate::core::{
	api_clients::{fabric_meta::FabricMetaAPIClient, piston_meta::PistonMetaAPIClient},
	bin_providers::MCJEDownloadInfo,
};

pub struct FabricBinaryProvider {
	fabric_meta: FabricMetaAPIClient,
	piston_meta: PistonMetaAPIClient,
}

impl FabricBinaryProvider {
	pub fn new(fabric_meta: FabricMetaAPIClient, piston_meta: PistonMetaAPIClient) -> Self {
		Self {
			fabric_meta,
			piston_meta,
		}
	}

	pub async fn get_latest(&self, pre_release: bool) -> Result<MCJEDownloadInfo, String> {
		let manifest = self.fabric_meta.get_manifest().await?;

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
			.find(|v| v.stable != pre_release)
			.ok_or("No suitable game versions found")?;

		self.get(
			&latest_game.version,
			&latest_loader.version,
			&latest_installer.version,
		)
		.await
	}

	pub async fn get(
		&self,
		game_version: &str,
		loader_version: &str,
		launcher_version: &str,
	) -> Result<MCJEDownloadInfo, String> {
		let version_info = self
			.fabric_meta
			.get_version(game_version, loader_version)
			.await;

		if let Err(err) = version_info {
			return Err(format!(
				"Failed to verify fabric version {} exists: {}",
				game_version, err
			));
		}

		let java_version = self
			.piston_meta
			.get_version(game_version)
			.await
			.map_err(|e| format!("Cannot query version {} from Mojang: {}", game_version, e))?
			.java_version
			.major_version;

		let download_url = self
			.fabric_meta
			.get_download_url(game_version, loader_version, launcher_version)
			.await?;

		let download_info = MCJEDownloadInfo {
			download_url,
			file_name: "server.jar".to_string(),
			hash: None,
			java_version,
			java_args: vec![],
		};

		Ok(download_info)
	}

	pub async fn list_game_versions(&self) -> Result<Vec<String>, String> {
		let manifest = self.fabric_meta.get_manifest().await?;
		let versions: Vec<String> = manifest.game.iter().map(|v| v.version.clone()).collect();

		Ok(versions)
	}

	pub async fn list_loader_versions(&self, game_version: &str) -> Result<Vec<String>, String> {
		let loaders = self
			.fabric_meta
			.get_versions(game_version)
			.await
			.map_err(|e| format!("Failed to get loader versions: {}", e))?;

		let versions: Vec<String> = loaders
			.iter()
			.map(|loader| loader.loader.version.clone())
			.collect();

		Ok(versions)
	}

	pub async fn list_installer_versions(&self) -> Result<Vec<String>, String> {
		let manifest = self.fabric_meta.get_manifest().await?;

		let installers: Vec<String> = manifest
			.installer
			.iter()
			.map(|installer| installer.version.clone())
			.collect();

		Ok(installers)
	}
}
