use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;

#[derive(Error, Debug)]
pub enum JavaError {
	#[error("Java not found: {0}")]
	NotFound(String),
	#[error("Encountered FS error: {0}")]
	FileSystem(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JavaVersion {
	pub major_version: u8,
	pub version_string: String,
	pub vendor: String,
	pub arch: String,
}

/// Retrieves a suitable Java version for the specified major version.
/// Returns the first matching version found.
pub async fn get_suitable_for(major_version: u8) -> Result<JavaVersion, JavaError> {
	let javas = get_versions().await?;
	let suitable = javas
		.into_iter()
		.find(|java| java.major_version == major_version);

	suitable.ok_or(JavaError::NotFound(format!(
		"No suitable Java version found for major version {major_version}"
	)))
}

/// Parses the Java properties output to extract Java version information.
fn parse_java_output(output: &str) -> JavaVersion {
	let mut major_version: Option<u8> = None;
	let mut version_string: Option<String> = None;
	let mut vendor: Option<String> = None;
	let mut arch: Option<String> = None;

	for line in output.lines() {
		let line = line.trim();

		if let Some(rest) = line.strip_prefix("java.specification.version =") {
			let trimmed = rest.trim();

			// Handle legacy version strings by stripping "1." prefix
			let version_part = if trimmed.starts_with("1.") {
				trimmed.trim_start_matches("1.")
			} else {
				trimmed
			};

			let parsed_version = version_part.parse::<u8>();

			if let Ok(version) = parsed_version {
				major_version = Some(version);
			} else {
				tracing::warn!(
					"Failed to parse major version from JVM output: {}",
					rest.trim()
				);
			}
		}

		if let Some(rest) = line.strip_prefix("java.vm.version =") {
			version_string = Some(rest.trim().to_string());
		}

		if let Some(rest) = line.strip_prefix("java.vm.name =") {
			vendor = Some(rest.trim().to_string());
		}

		if let Some(rest) = line.strip_prefix("os.arch =") {
			arch = Some(rest.trim().to_string());
		}
	}

	JavaVersion {
		major_version: major_version.unwrap_or(0),
		version_string: version_string.unwrap_or_else(|| "Unknown".into()),
		vendor: vendor.unwrap_or_else(|| "Unknown".into()),
		arch: arch.unwrap_or_else(|| "Unknown".into()),
	}
}

/// Retrieves all available Java versions installed on the system.
pub async fn get_versions() -> Result<Vec<JavaVersion>, JavaError> {
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

	let mut javas: Vec<JavaVersion> = Vec::new();

	for jvm in jvm_paths {
		let command_output = Command::new(&jvm)
			.arg("-XshowSettings:properties")
			.arg("-version")
			.output()
			.await;

		if let Err(err) = command_output {
			tracing::warn!("Failed to run command on JVM {}: {}", jvm.display(), err);
			continue;
		}

		let probe_output = command_output.unwrap();
		let output = String::from_utf8_lossy(&probe_output.stderr); // Java version info is printed to stderr
		let version = parse_java_output(&output);
		javas.push(version);
	}

	Ok(javas)
}
