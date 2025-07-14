use super::version::{fabric::FabricVersionInfo, mojang_java::MojangJavaVersionInfo, VersionInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Game {
	MCJava { version: MojangJavaVersionInfo },
	MCJavaFabric { version: FabricVersionInfo },
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MCJava { .. } => "minecraft-java",
			Game::MCJavaFabric { .. } => "minecraft-java-fabric",
		}
	}

	pub fn version(&self) -> &dyn VersionInfo {
		match self {
			Game::MCJava { version } => version,
			Game::MCJavaFabric { version } => version,
		}
	}

	pub fn from_path_parts(game_type: &str, version_str: &str) -> Result<Self, String> {
		match game_type {
			"minecraft-java" => {
				let version = MojangJavaVersionInfo::from_identifier(version_str)?;
				Ok(Game::MCJava { version })
			}
			"minecraft-java-fabric" => {
				let version = FabricVersionInfo::from_identifier(version_str)?;
				Ok(Game::MCJavaFabric { version })
			}
			_ => Err(format!("Unknown game type: {}", game_type)),
		}
	}
}
