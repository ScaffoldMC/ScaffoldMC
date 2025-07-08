use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Game {
	MCJava {
		version: String,
	},
	MCJavaFabric {
		mc_version: String,
		fabric_version: String,
	},
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MCJava { .. } => "minecraft-java",
			Game::MCJavaFabric { .. } => "minecraft-java-fabric",
		}
	}

	/// Get the version of this game as a single string
	pub fn version(&self) -> String {
		match self {
			Game::MCJava { version } => version.clone(),
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => format!("{mc_version}-{fabric_version}"),
		}
	}
}
