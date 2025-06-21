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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ServerConfig {
	pub name: String,
	pub version: GameVersion,
	pub jvm_args: Vec<String>,
}
