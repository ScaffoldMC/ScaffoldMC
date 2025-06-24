use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "name", rename_all = "snake_case")]
enum Loader {
	Fabric { version: String },
	Paper { version: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "game", rename_all = "snake_case")]
enum GameVersion {
	MinecraftJava {
		version: String,
		loader: Option<Loader>,
	},
	MinecraftBedrock {
		version: String,
	},
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	#[error("Failed to read config file: {0}")]
	Io(#[from] std::io::Error),
	#[error("Failed to parse config: {0}")]
	Parse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ServerConfig {
	pub name: String,
	pub version: GameVersion,
	pub jvm_args: Vec<String>,
}

impl ServerConfig {
	pub fn load_from_file(path: PathBuf) -> Result<Self, ConfigError> {
		let file = std::fs::read_to_string(path)?;

		Ok(toml::from_str(&file)?)
	}
}
