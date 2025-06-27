use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
	pub version: super::game::Game,
	pub jvm_args: Vec<String>,
}

impl ServerConfig {
	pub fn load_from_file(path: PathBuf) -> Result<Self, ConfigError> {
		let file = std::fs::read_to_string(path)?;

		Ok(toml::from_str(&file)?)
	}
}
