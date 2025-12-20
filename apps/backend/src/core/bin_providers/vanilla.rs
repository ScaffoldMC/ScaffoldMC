use crate::{
	core::{api_clients::piston_meta::PistonMetaAPIClient, bin_providers::JavaDownloadInfo},
	util::hash::HashAlgorithm,
};
use reqwest::Url;

pub struct VanillaBinaryProvider {
	api_client: PistonMetaAPIClient,
}

impl VanillaBinaryProvider {
	pub fn new(api_client: PistonMetaAPIClient) -> Self {
		Self { api_client }
	}

	async fn get_latest(&self, pre_release: bool) -> Result<JavaDownloadInfo, String> {
		let manifest = self.api_client.get_manifest().await?;

		let latest_version = if pre_release {
			manifest.latest.snapshot
		} else {
			manifest.latest.release
		};

		self.get(&latest_version).await
	}

	async fn get(&self, game_version: &str) -> Result<JavaDownloadInfo, String> {
		let version_info = self.api_client.get_version(game_version).await?;

		let download_url = Url::parse(&version_info.downloads.server.url)
			.map_err(|e| format!("Failed to parse URL: {}", e))?;
		let hash = version_info.downloads.server.sha1.clone();

		let java_version = version_info.java_version.major_version;

		let download_info = JavaDownloadInfo {
			download_url,
			file_name: "server.jar".to_string(),
			hash: Some((hash, HashAlgorithm::Sha1)),
			java_version,
			java_args: vec![],
		};

		Ok(download_info)
	}

	async fn list_versions(&self) -> Result<Vec<String>, String> {
		let manifest = self.api_client.get_manifest().await?;

		let mut listings: Vec<String> = Vec::new();

		for v in &manifest.versions {
			listings.push(v.id.clone());
		}

		Ok(listings)
	}
}
