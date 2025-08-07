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
	let jvm_paths: Vec<PathBuf> = {
		let mut dirs = Vec::new();

		if let Ok(java_home) = std::env::var("JAVA_HOME") {
			dirs.push(PathBuf::from(java_home));
		}

		#[cfg(all(target_family = "unix", not(target_os = "macos")))]
		{
			dirs.push(PathBuf::from("/usr/lib/jvm"));
			if let Ok(entries) = std::fs::read_dir("/usr/lib/jvm") {
				for entry in entries.flatten() {
					let java_bin = entry.path().join("bin/java");
					if java_bin.exists() {
						dirs.push(java_bin);
					}
				}
			}
		}

		#[cfg(target_os = "macos")]
		{
			let java_dir = PathBuf::from("/Library/Java/JavaVirtualMachines");

			if !java_dir.exists() && !java_dir.is_dir() {
				return Err(JavaError::FileSystem(
					"Java directory does not exist or is not a directory".to_string(),
				));
			}

			if let Ok(entries) = std::fs::read_dir(&java_dir) {
				for entry in entries.flatten() {
					let java_bin = entry.path().join("Contents/Home/bin/java");
					if java_bin.exists() {
						dirs.push(java_bin);
					}
				}
			}
		}

		#[cfg(target_os = "windows")]
		{
			let java_dirs = vec![
				PathBuf::from(r"C:\Program Files\Java"),
				PathBuf::from(r"C:\Program Files (x86)\Java"),
			];

			for base_dir in &java_dirs {
				if let Ok(entries) = std::fs::read_dir(&base_dir) {
					for entry in entries.flatten() {
						let java_bin = entry.path().join("bin\\java.exe");
						if java_bin.exists() {
							dirs.push(java_bin);
						}
					}
				}
			}
		}

		dirs
	};

	let javas: Vec<JavaVersion> = Vec::new();

	for jvm in jvm_paths {
		// TODO: Run Java probe to get java info
	}

	Ok(javas)
}
