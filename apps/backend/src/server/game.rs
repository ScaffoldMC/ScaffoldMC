use serde::{Deserialize, Serialize};

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2/";

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
	async fn install_fabric(mc_version: &str, fabric_version: &str) -> Result<(), String> {
		let url = format!(
			"{}versions/loader/{}/{}/server/jar",
			FABRIC_API_URL, mc_version, fabric_version
		);

		let response = reqwest::get(&url)
			.await
			.map_err(|e| format!("Failed to download: {}", e))?;

		let bytes = response
			.bytes()
			.await
			.map_err(|e| format!("Failed to read response: {}", e))?;

		// TODO: Centralize the path where the server jar is saved
		std::fs::write("data/server.jar", bytes)
			.map_err(|e| format!("Failed to save file: {}", e))?;

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
						} => Self::install_fabric(mc_version, loader_version).await?,
						Loader::Paper {
							version: loader_version,
						} => todo!(),
					}
				} else {
					// TODO: Logic to install Java Minecraft
					println!("Installing Minecraft Java {}", mc_version);
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
