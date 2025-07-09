use crate::core::version::VersionInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MojangJavaVersionInfo {
	game_version: String,
	is_prerelease: bool,
}

impl MojangJavaVersionInfo {
	pub fn new(game_version: String, is_prerelease: bool) -> Self {
		Self {
			game_version,
			is_prerelease,
		}
	}
}

impl VersionInfo for MojangJavaVersionInfo {
	fn game_version(&self) -> &str {
		&self.game_version
	}

	fn is_prerelease(&self) -> bool {
		self.is_prerelease
	}

	fn identifier(&self) -> String {
		self.game_version.clone()
	}
}
