pub mod java;

use java::MinecraftJava;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Clone, Deserialize, Serialize)]
#[ts(export)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Game {
	MinecraftJava(MinecraftJava),
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MinecraftJava { .. } => "minecraft-java",
		}
	}

	pub fn version_string(&self) -> String {
		match self {
			Game::MinecraftJava(minecraft_java) => format!(
				"{}-{}",
				minecraft_java.version,
				minecraft_java.loader.version_string()
			),
		}
	}
}
