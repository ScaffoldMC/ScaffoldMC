use crate::core::version::VersionInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MojangJavaVersionInfo {
	game: String,
}

impl MojangJavaVersionInfo {
	pub fn new(game: String) -> Self {
		Self { game }
	}
}

impl VersionInfo for MojangJavaVersionInfo {
	fn game(&self) -> &str {
		&self.game
	}

	fn identifier(&self) -> String {
		self.game.clone()
	}

	fn from_identifier(identifier: &str) -> Result<Self, String>
	where
		Self: Sized,
	{
		if identifier.is_empty() {
			return Err("Identifier cannot be empty".to_string());
		}
		Ok(Self {
			game: identifier.to_string(),
		})
	}
}
