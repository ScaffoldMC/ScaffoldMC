use super::{VersionInfo, VersionInfoConstructor};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FabricVersionInfo {
	game: String,
	fabric: String,
	launcher: String,
}

impl FabricVersionInfo {
	pub fn new(game: String, fabric: String, launcher: String) -> Self {
		Self {
			game,
			fabric,
			launcher,
		}
	}

	pub fn fabric(&self) -> &str {
		&self.fabric
	}

	pub fn launcher(&self) -> &str {
		&self.launcher
	}
}

impl VersionInfo for FabricVersionInfo {
	fn game(&self) -> &str {
		&self.game
	}

	fn identifier(&self) -> String {
		format!("{}-{}-{}", self.game, self.fabric, self.launcher)
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

impl VersionInfoConstructor for FabricVersionInfo {
	type VersionType = FabricVersionInfo;
	
	fn from_identifier(identifier: &str) -> Result<Self::VersionType, String> {
		let parts: Vec<&str> = identifier.split('-').collect();
		if parts.len() != 3 {
			return Err(format!("Invalid identifier format: {}", identifier));
		}

		Ok(Self::new(
			parts[0].to_string(),
			parts[1].to_string(),
			parts[2].to_string(),
		))
	}
}
