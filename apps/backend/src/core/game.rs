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
}
