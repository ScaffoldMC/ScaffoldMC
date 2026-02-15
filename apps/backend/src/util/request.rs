use reqwest::IntoUrl;

pub async fn get_and_format<U: IntoUrl, T: serde::de::DeserializeOwned>(
	client: &reqwest::Client,
	url: U,
) -> Result<T, String> {
	let response = client
		.get(url)
		.send()
		.await
		.map_err(|e| format!("Failed to fetch versions: {e}"))?;

	let text = response
		.text()
		.await
		.map_err(|e| format!("Failed to read response body: {e}"))?;

	let decoded: T =
		serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {e}"))?;

	Ok(decoded)
}
