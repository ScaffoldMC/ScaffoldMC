use crate::core::bin_providers::{BinaryListing, BinaryProvider};

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

struct FabricBinaryListing {
	download_url: String,
	version: String,
}

impl FabricBinaryListing {
	pub fn new(version: String) -> Self {
		Self {
			download_url: format!("{FABRIC_API_URL}/versions/game/{version}"),
			version,
		}
	}
}

impl BinaryListing for FabricBinaryListing {
	fn download_url(&self) -> &str {
		&self.download_url
	}

	fn version(&self) -> &str {
		&self.version
	}

	fn file_name(&self) -> &str {
		&self.version
	}
}

pub struct FabricBinaryProvider;

impl BinaryProvider<FabricBinaryListing> for FabricBinaryProvider {
	async fn list_all(&self) -> Result<Vec<FabricBinaryListing>, String> {
		todo!()
	}

	async fn latest(&self) -> Result<FabricBinaryListing, String> {
		todo!()
	}
}
