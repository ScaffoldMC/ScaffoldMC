use serde::{Deserialize, Serialize};

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
	pub fn install(&self) -> Result<(), String> {
		match self {
			Game::MinecraftJava { version, loader } => {
				if let Some(loader) = loader {
					// TODO: Logic to install Java Minecraft with a loader
					println!(
						"Installing Minecraft Java {} with loader {:?}",
						version, loader
					);
				} else {
					// TODO: Logic to install Java Minecraft
					println!("Installing Minecraft Java {}", version);
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
