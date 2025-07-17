use crate::core::game::Game;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	#[error("Failed to read config file: {0}")]
	Io(#[from] std::io::Error),
	#[error("Failed to parse config: {0}")]
	Parse(#[from] toml::de::Error),
	#[error("Failed to serialize config: {0}")]
	Serialize(#[from] toml::ser::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ServerConfig {
	pub name: String,
	pub game: Game,
	pub args: Vec<String>,
	pub stop_command: String,
}

impl ServerConfig {
	pub fn load_from_file(path: PathBuf) -> Result<Self, ConfigError> {
		let file = std::fs::read_to_string(path)?;

		Ok(toml::from_str(&file)?)
	}

	pub fn save_to_file(&self, path: PathBuf) -> Result<(), ConfigError> {
		let toml_string = toml::to_string(self)?;
		std::fs::write(path, toml_string)?;
		Ok(())
	}
}
