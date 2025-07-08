use crate::core::util::mojang_api::get_version_info;
use serde::{Deserialize, Serialize};

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Game {
	MCJava {
		version: String,
	},
	MCJavaFabric {
		mc_version: String,
		fabric_version: String,
	},
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MCJava { version } => "minecraft-java",
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => "minecraft-java-fabric",
		}
	}

	/// Get the download URL for this game
	pub async fn get_download_url(&self) -> String {
		match self {
			Game::MCJava { version } => get_version_info(version)
				.await
				.expect("Failed to get version info") // TODO: Handle error properly
				.url
				.clone(),
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => {
				format!("{FABRIC_API_URL}/versions/loader/{mc_version}/{fabric_version}/server/jar")
			}
		}
	}

	/// Get the name of the binary for this game
	pub fn binary_name(&self) -> &str {
		match self {
			Game::MCJava { version } => "server.jar",
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => "server.jar",
		}
	}

	/// Get the version of this game as a single string
	pub fn version(&self) -> String {
		match self {
			Game::MCJava { version } => version.clone(),
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => format!("{mc_version}-{fabric_version}"),
		}
	}
}
