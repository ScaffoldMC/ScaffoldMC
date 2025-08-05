use crate::core::version::{VersionInfo, VersionInfoConstructor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VanillaVersionInfo {
	game: String,
}

impl VanillaVersionInfo {
	pub fn new(game: String) -> Self {
		Self { game }
	}
}

impl VersionInfo for VanillaVersionInfo {
	fn game(&self) -> &str {
		&self.game
	}

	fn identifier(&self) -> String {
		self.game.clone()
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

impl VersionInfoConstructor for VanillaVersionInfo {
	type VersionType = VanillaVersionInfo;

	fn from_identifier(identifier: &str) -> Result<Self::VersionType, String> {
		if identifier.is_empty() {
			return Err("Identifier cannot be empty".to_string());
		}
		Ok(Self {
			game: identifier.to_string(),
		})
	}
}
