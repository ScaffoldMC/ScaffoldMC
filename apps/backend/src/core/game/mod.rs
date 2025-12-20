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
}
