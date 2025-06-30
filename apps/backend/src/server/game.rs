use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::server::mojang_api::get_version_info;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Loader {
	Fabric { version: String },
	Paper { version: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "game", rename_all = "snake_case")]
pub enum Game {
	MinecraftJava {
		version: String,
		loader: Option<Loader>,
	},
	MinecraftBedrock {
		version: String,
	},
}

impl Game {
	async fn download_file(url: &str, path: PathBuf) -> Result<(), String> {
		let response = reqwest::get(url)
			.await
			.map_err(|e| format!("Failed to download: {}", e))?;

		if !response.status().is_success() {
			return Err(format!(
				"Failed to download file from {}: {}",
				url,
				response.status()
			));
		}

		let bytes = response
			.bytes()
			.await
			.map_err(|e| format!("Failed to read response: {}", e))?;

		std::fs::write(path, bytes).map_err(|e| format!("Failed to save file: {}", e))?;

		Ok(())
	}

	pub async fn install(&self) -> Result<(), String> {
		match self {
			Game::MinecraftJava {
				version: mc_version,
				loader,
			} => {
				if let Some(loader) = loader {
					match loader {
						Loader::Fabric {
							version: loader_version,
						} => {
							let url = format!(
								"{FABRIC_API_URL}/versions/loader/{mc_version}/{loader_version}/server/jar"
							);

							let install_path = PathBuf::from(format!(
								"games/mcje/{mc_version}/fabric/{loader_version}/server.jar"
							));

							Self::download_file(&url, install_path).await?;
						}
						Loader::Paper {
							version: loader_version,
						} => todo!(),
					}
				} else {
					let version_info = get_version_info(mc_version)
						.await
						.map_err(|e| e.to_string())?;

					let install_path =
						PathBuf::from(format!("games/mcje/{mc_version}/vanilla/server.jar"));

					Self::download_file(&version_info.url, install_path).await?;
				}
			}
			Game::MinecraftBedrock { version } => {
				// TODO: Logic to install Bedrock Minecraft
				println!("Installing Minecraft Bedrock {}", version);
			}
		}
		Ok(())
	}
}
