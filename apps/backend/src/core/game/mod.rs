pub mod java;

use java::MinecraftJava;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

	pub fn version_identifier(&self) -> String {
		match self {
			Game::MinecraftJava(minecraft_java) => format!(
				"{}-{}",
				minecraft_java.version,
				match &minecraft_java.loader {
					java::JavaRuntime::Vanilla => "vanilla".to_string(),
					java::JavaRuntime::Fabric { loader, launcher } => {
						format!("fabric-{}-{}", loader, launcher)
					}
					java::JavaRuntime::Paper { build } => format!("paper-{}", build),
				}
			),
		}
	}
}
