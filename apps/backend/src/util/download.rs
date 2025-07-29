use std::path::PathBuf;

use reqwest::Url;

/// Download a file from a URL.
pub async fn download_file(url: &Url, path: PathBuf) -> Result<(), String> {
	let response = reqwest::get(url.clone())
		.await
		.map_err(|e| format!("Failed to download: {}", e))?;

	if !response.status().is_success() {
		return Err(format!(
			"Failed to download file from {}: {}",
			url,
			response.status()
		));
	}

	let bytes = response
		.bytes()
		.await
		.map_err(|e| format!("Failed to read response: {}", e))?;

	std::fs::write(path, bytes).map_err(|e| format!("Failed to save file: {}", e))?;

	Ok(())
}
