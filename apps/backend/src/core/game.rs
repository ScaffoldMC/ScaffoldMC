use crate::core::version::{
	fabric::FabricVersionInfo, mojang_java::MojangJavaVersionInfo, paper::PaperVersionInfo,
	VersionInfo, VersionInfoConstructor,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Game {
	MinecraftJava { version: MojangJavaVersionInfo },
	MinecraftJavaFabric { version: FabricVersionInfo },
	MinecraftJavaPaper { version: PaperVersionInfo },
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MinecraftJava { .. } => "minecraft-java",
			Game::MinecraftJavaFabric { .. } => "minecraft-java-fabric",
			Game::MinecraftJavaPaper { .. } => "minecraft-java-paper",
		}
	}

	pub fn version(&self) -> Arc<dyn VersionInfo> {
		match self {
			Game::MinecraftJava { version } => Arc::new(version.clone()),
			Game::MinecraftJavaFabric { version } => Arc::new(version.clone()),
			Game::MinecraftJavaPaper { version } => Arc::new(version.clone()),
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
