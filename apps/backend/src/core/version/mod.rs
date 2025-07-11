pub mod fabric;
pub mod mojang_java;

pub trait VersionInfo {
	fn from_identifier(identifier: &str) -> Result<Self, String>
	where
		Self: Sized;

	fn game_version(&self) -> &str;
	fn is_prerelease(&self) -> bool;
	fn identifier(&self) -> String;
}
