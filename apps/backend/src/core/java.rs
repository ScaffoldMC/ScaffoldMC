use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JavaError {
	#[error("Java not found: {0}")]
	NotFound(String),
	#[error("Encountered FS error: {0}")]
	FileSystem(String),
}

pub struct JavaVersion {
	pub major_version: u8,
	pub version_string: String,
	pub identifier: String,
	pub path: String,
}

/// Retrieves a suitable Java version for the specified major version.
/// Returns the first matching version found.
pub fn get_suitable_for(major_version: u8) -> Result<JavaVersion, JavaError> {
	let javas = get_versions()?;
	let suitable = javas
		.into_iter()
		.find(|java| java.major_version == major_version);

	suitable.ok_or(JavaError::NotFound(format!(
		"No suitable Java version found for major version {}",
		major_version
	)))
}

/// Retrieves all available Java versions installed on the system.
pub fn get_versions() -> Result<Vec<JavaVersion>, JavaError> {
	let jres_path = PathBuf::from("/usr/lib/jvm");

	if !jres_path.exists() {
		return Err(JavaError::NotFound("No Java directory found".to_string()));
	}

	let dir_entries =
		std::fs::read_dir(&jres_path).map_err(|e| JavaError::FileSystem(e.to_string()))?;

	let javas: Vec<JavaVersion> = Vec::new();

	for entry in dir_entries {
		let entry = entry.map_err(|e| JavaError::FileSystem(e.to_string()))?;
		let path = entry.path();

		if !path.is_dir() {
			continue;
		}

		// TODO: Run Java probe to get java info
	}

	Ok(javas)
}
