use super::version::{fabric::FabricVersionInfo, mojang_java::MojangJavaVersionInfo, VersionInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Game {
	MinecraftJava { version: MojangJavaVersionInfo },
	MinecraftJavaFabric { version: FabricVersionInfo },
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MinecraftJava { .. } => "minecraft-java",
			Game::MinecraftJavaFabric { .. } => "minecraft-java-fabric",
		}
	}

	pub fn version(&self) -> &dyn VersionInfo {
		match self {
			Game::MinecraftJava { version } => version,
			Game::MinecraftJavaFabric { version } => version,
		}
	}

	pub fn from_path_parts(game_type: &str, version_str: &str) -> Result<Self, String> {
		match game_type {
			"minecraft-java" => {
				let version = MojangJavaVersionInfo::from_identifier(version_str)?;
				Ok(Game::MinecraftJava { version })
			}
			"minecraft-java-fabric" => {
				let version = FabricVersionInfo::from_identifier(version_str)?;
				Ok(Game::MinecraftJavaFabric { version })
			}
			_ => Err(format!("Unknown game type: {}", game_type)),
		}
	}
}
