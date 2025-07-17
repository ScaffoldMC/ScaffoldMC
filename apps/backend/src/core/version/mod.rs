pub mod fabric;
pub mod mojang_java;

pub trait VersionInfo {
	fn from_identifier(identifier: &str) -> Result<Self, String>
	where
		Self: Sized;

	fn game(&self) -> &str;
	fn identifier(&self) -> String;
}
