use crate::core::bin_providers::{BinaryListing, BinaryProvider};

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

struct FabricBinaryListing {
	download_url: String,
	version: String,
}

impl FabricBinaryListing {
	pub fn new(version: String, download_url: String) -> Self {
		Self {
			download_url,
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

impl BinaryProvider for FabricBinaryProvider {
	type Listing = FabricBinaryListing;

	fn new() -> Self {
		Self {}
	}

	async fn list_all(&self) -> Result<Vec<Self::Listing>, String> {
		todo!()
	}

	async fn latest(&self) -> Result<Self::Listing, String> {
		todo!()
	}

	async fn get(&self, version: &str) -> Result<Self::Listing, String> {
		Ok(FabricBinaryListing::new(
			version.into(),
			format!("{FABRIC_API_URL}/versions/game/{version}"),
		))
	}
}
