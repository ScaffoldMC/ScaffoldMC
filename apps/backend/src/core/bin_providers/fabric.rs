use crate::core::{api_clients::fabric::FabricMetaAPIClient, bin_providers::JavaDownloadInfo};

pub struct FabricBinaryProvider {
	api_client: FabricMetaAPIClient,
}

impl FabricBinaryProvider {
	pub fn new(api_client: FabricMetaAPIClient) -> Self {
		Self { api_client }
	}

	pub async fn get_latest(&self, pre_release: bool) -> Result<JavaDownloadInfo, String> {
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
	) -> Result<JavaDownloadInfo, String> {
		let version_info = self
			.api_client
			.get_version(game_version, loader_version)
			.await?;

		let java_version = version_info.launcher_meta.min_java_version;

		let download_url = self
			.api_client
			.get_download_url(game_version, loader_version, launcher_version)
			.await?;

		let download_info = JavaDownloadInfo {
			download_url,
			file_name: "server.jar".to_string(),
			hash: None,
			java_version: java_version,
			java_args: vec![],
		};

		Ok(download_info)
	}

	pub async fn list_game_versions(&self) -> Result<Vec<String>, String> {
		let manifest = self.api_client.get_manifest().await?;
		let versions: Vec<String> = manifest.game.iter().map(|v| v.version.clone()).collect();

		Ok(versions)
	}

	pub async fn list_loader_versions(&self, game_version: &str) -> Result<Vec<String>, String> {
		let loaders = self
			.api_client
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
		let manifest = self.api_client.get_manifest().await?;

		let installers: Vec<String> = manifest
			.installer
			.iter()
			.map(|installer| installer.version.clone())
			.collect();

		Ok(installers)
	}
}
