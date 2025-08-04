use reqwest::IntoUrl;

pub async fn get_and_format<U: IntoUrl, T: serde::de::DeserializeOwned>(
	client: &reqwest::Client,
	url: U,
) -> Result<T, String> {
	let response = client
		.get(url)
		.send()
		.await
		.map_err(|e| format!("Failed to fetch versions: {}", e))?
		.json::<T>()
		.await
		.map_err(|e| format!("Failed to parse response: {}", e))?;

	Ok(response)
}
