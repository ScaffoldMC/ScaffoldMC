use super::VersionInfo;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FabricVersionInfo {
	game_version: String,
	fabric_version: String,
	launcher_version: String,
}

impl FabricVersionInfo {
	pub fn new(game_version: String, fabric_version: String, launcher_version: String) -> Self {
		Self {
			game_version,
			fabric_version,
			launcher_version,
		}
	}

	pub fn fabric_version(&self) -> &str {
		&self.fabric_version
	}

	pub fn launcher_version(&self) -> &str {
		&self.launcher_version
	}
}

impl VersionInfo for FabricVersionInfo {
	fn game_version(&self) -> &str {
		&self.game_version
	}

	fn identifier(&self) -> String {
		format!(
			"{}-{}-{}",
			self.game_version, self.fabric_version, self.launcher_version
		)
	}

	fn from_identifier(identifier: &str) -> Result<Self, String>
	where
		Self: Sized,
	{
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
