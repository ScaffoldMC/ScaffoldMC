use crate::core::version::VersionInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MojangJavaVersionInfo {
	game_version: String,
}

impl MojangJavaVersionInfo {
	pub fn new(game_version: String) -> Self {
		Self { game_version }
	}
}

impl VersionInfo for MojangJavaVersionInfo {
	fn game_version(&self) -> &str {
		&self.game_version
	}

	fn identifier(&self) -> String {
		self.game_version.clone()
	}

	fn from_identifier(identifier: &str) -> Result<Self, String>
	where
		Self: Sized,
	{
		if identifier.is_empty() {
			return Err("Identifier cannot be empty".to_string());
		}
		Ok(Self {
			game_version: identifier.to_string(),
		})
	}
}
