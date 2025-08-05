pub mod fabric;
pub mod paper;
pub mod vanilla;

pub trait VersionInfo: Send + Sync {
	fn game(&self) -> &str;
	fn identifier(&self) -> String;

	/// Returns an Any reference for downcasting to specific version types
	fn as_any(&self) -> &dyn std::any::Any;
}

pub trait VersionInfoConstructor {
	type VersionType: VersionInfo;

	fn from_identifier(identifier: &str) -> Result<Self::VersionType, String>;
}
