use crate::{
	core::{api_clients::paper::PaperDownloadsAPIClient, bin_providers::JavaDownloadInfo},
	util::hash::HashAlgorithm,
};
use reqwest::Url;

pub struct PaperBinaryProvider {
	api_client: PaperDownloadsAPIClient,
}

impl PaperBinaryProvider {
	pub fn new(api_client: PaperDownloadsAPIClient) -> Self {
		Self { api_client }
	}

	pub async fn get_latest(&self) -> Result<JavaDownloadInfo, String> {
		let versions = self.api_client.get_versions().await?;

		let latest_version = versions.versions.first().ok_or("No versions found")?;

		self.get(latest_version.version.id.as_str(), latest_version.builds[0])
			.await
	}

	pub async fn get(&self, game_version: &str, build: u16) -> Result<JavaDownloadInfo, String> {
		let response = self.api_client.get_version(game_version).await?;
		let java_version = response.version.java.version.minimum;
		let java_args = response.version.java.flags.recommended;

		let build = self.api_client.get_build(game_version, build).await?;

		let download_url = Url::parse(&build.downloads.server_default.url)
			.map_err(|e| format!("Failed to parse download URL: {}", e))?;

		let hash = build.downloads.server_default.checksums.sha256;

		let download_info = JavaDownloadInfo {
			download_url,
			file_name: "server.jar".to_string(),
			hash: Some((hash, HashAlgorithm::Sha256)),
			java_version,
			java_args,
		};

		Ok(download_info)
	}

	pub async fn list_game_versions(&self) -> Result<Vec<String>, String> {
		let response = self.api_client.get_versions().await?;
		let mut versions: Vec<String> = Vec::new();

		for version in response.versions {
			versions.push(version.version.id);
		}

		Ok(versions)
	}

	pub async fn list_loader_versions(&self, game_version: &str) -> Result<Vec<u16>, String> {
		let response = self.api_client.get_versions().await?;

		let version = response
			.versions
			.iter()
			.find(|v| v.version.id == game_version)
			.ok_or("Game version not found")?;

		Ok(version.builds.clone())
	}
}
