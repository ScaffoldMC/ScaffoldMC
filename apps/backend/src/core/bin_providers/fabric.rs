use crate::core::bin_providers::{BinaryInfo, BinaryProvider, VersionInfo};
use reqwest::Url;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

pub struct FabricVersionInfo {
	game_version: String,
	fabric_version: String,
	launcher_version: String,
	is_prerelease: bool,
}

impl FabricVersionInfo {
	pub fn new(
		game_version: String,
		fabric_version: String,
		launcher_version: String,
		is_prerelease: bool,
	) -> Self {
		Self {
			game_version,
			fabric_version,
			launcher_version,
			is_prerelease,
		}
	}

	fn fabric_version(&self) -> &str {
		&self.fabric_version
	}

	fn launcher_version(&self) -> &str {
		&self.launcher_version
	}
}

impl VersionInfo for FabricVersionInfo {
	fn game_version(&self) -> &str {
		&self.game_version
	}

	fn is_prerelease(&self) -> bool {
		self.is_prerelease
	}

	fn identifier(&self) -> String {
		format!(
			"{}-{}-{}",
			self.game_version, self.fabric_version, self.launcher_version
		)
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

impl BinaryProvider for FabricBinaryProvider {
	type Binary = FabricBinaryInfo;

	fn new() -> Self {
		Self {}
	}

	fn binary_name(&self) -> &str {
		"server.jar"
	}

	async fn list_versions(&self) -> Result<Vec<FabricVersionInfo>, String> {
		todo!()
	}

	async fn get_latest(&self, pre_release: bool) -> Result<Self::Binary, String> {
		todo!()
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
