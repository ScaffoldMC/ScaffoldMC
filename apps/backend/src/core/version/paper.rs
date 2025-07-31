use super::{VersionInfo, VersionInfoConstructor};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaperVersionInfo {
	game: String,
	paper_build: u16,
}

impl PaperVersionInfo {
	pub fn new(game: String, paper_build: u16) -> Self {
		Self { game, paper_build }
	}

	pub fn paper_build(&self) -> u16 {
		self.paper_build
	}
}

impl VersionInfo for PaperVersionInfo {
	fn game(&self) -> &str {
		&self.game
	}

	fn identifier(&self) -> String {
		format!("{}-{}", self.game, self.paper_build)
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

impl VersionInfoConstructor for PaperVersionInfo {
	type VersionType = PaperVersionInfo;

	fn from_identifier(identifier: &str) -> Result<Self::VersionType, String> {
		let parts: Vec<&str> = identifier.split('-').collect();
		if parts.len() != 2 {
			return Err(format!("Invalid identifier format: {}", identifier));
		}

		Ok(Self::new(
			parts[0].to_string(),
			parts[1]
				.parse()
				.map_err(|e| format!("Failed to parse paper build: {}", e))?,
		))
	}
}
