use crate::api_clients::fabric_meta::FabricMetaAPIClient;
use crate::api_clients::paper_meta::PaperDownloadsAPIClient;
use crate::api_clients::piston_meta::PistonMetaAPIClient;
use crate::bin_providers::paper::PaperBinaryProvider;
use crate::bin_providers::DownloadInfo;
use crate::bin_providers::{fabric::FabricBinaryProvider, vanilla::VanillaBinaryProvider};
use crate::models::file_schemas::binaries_lockfile::{
	BinaryLockfile, BinaryLockfileEntry, BinaryLockfileHash,
};
use crate::models::game::java::MinecraftJavaLoader;
use crate::models::game::Game;
use crate::models::hash::compute_file_hash;
use crate::services::Service;
use reqwest::Url;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BinaryService {
	binaries_dir: String,
	pub fabric: FabricBinaryProvider,
	pub mcje: VanillaBinaryProvider,
	pub paper: PaperBinaryProvider,
	lockfile_mutex: Arc<Mutex<()>>,
	reqwest_client: reqwest::Client,
}

impl Service for BinaryService {}

/// Service for managing game binaries.
impl BinaryService {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		let fabric_api = FabricMetaAPIClient::new(reqwest_client.clone());
		let paper_api = PaperDownloadsAPIClient::new(reqwest_client.clone());
		let piston_meta_api = PistonMetaAPIClient::new(reqwest_client.clone());

		Self {
			binaries_dir: format!("{}/games", crate::config::DATA_FOLDER),
			fabric: FabricBinaryProvider::new(fabric_api, piston_meta_api.clone()),
			mcje: VanillaBinaryProvider::new(piston_meta_api),
			paper: PaperBinaryProvider::new(paper_api),
			lockfile_mutex: Arc::new(Mutex::new(())),
			reqwest_client,
		}
	}

	/// Get information about a specific game version.
	pub async fn get_bin_info(&self, game: &Game) -> Result<DownloadInfo, String> {
		match game {
			Game::MinecraftJava(minecraft_java) => match &minecraft_java.loader {
				MinecraftJavaLoader::Vanilla => {
					let info = self.mcje.get(&minecraft_java.version).await?;
					Ok(info)
				}
				MinecraftJavaLoader::Fabric { loader, launcher } => {
					let info = self
						.fabric
						.get(&minecraft_java.version, loader, launcher)
						.await?;
					Ok(info)
				}
				MinecraftJavaLoader::Paper { build } => {
					let info = self.paper.get(&minecraft_java.version, *build).await?;
					Ok(info)
				}
			},
		}
	}

	/// Installs a game with the specified configuration.
	pub async fn install_game(&self, game: &Game) -> Result<PathBuf, String> {
		let _lock = self.lockfile_mutex.lock().await;
		let binary_dir = self.binary_dir(game);

		// Ensure the binary directory exists
		if !binary_dir.exists() {
			std::fs::create_dir_all(&binary_dir)
				.map_err(|e| format!("Failed to create binary directory: {e}"))?;
		}

		let download_info = self.get_bin_info(game).await?;
		let binary_path = binary_dir.join(download_info.file_name);

		Self::download_file(
			self.reqwest_client.clone(),
			&download_info.download_url,
			binary_path.clone(),
		)
		.await
		.map_err(|e| format!("Failed to download game: {e}"))?;

		// Add binary to the lockfile
		let mut lockfile = self
			.load_lockfile()
			.await
			.map_err(|e| format!("Failed to load lockfile: {e}"))?;

		let mut lockfile_entry = BinaryLockfileEntry {
			game: game.clone(),
			path: binary_path.clone(),
			hash: None,
		};

		if let Some((hash, hash_algorithm)) = download_info.hash {
			lockfile_entry.hash = Some(BinaryLockfileHash {
				algorithm: hash_algorithm,
				hash: hash.clone(),
			});
		}

		lockfile
			.binaries
			.insert(game.identifier().to_string(), lockfile_entry);

		self.save_lockfile(&lockfile).await?;

		Ok(binary_path)
	}

	/// Returns the path to the binary directory for a given game, if not
	/// available it will be downloaded.
	pub async fn ensure_binary(&self, game: &Game) -> Result<PathBuf, String> {
		let lock = self.lockfile_mutex.lock().await; // Lock for entire operation
		let lockfile = self.load_lockfile().await?;

		if let Some(entry) = lockfile.binaries.get(game.identifier()) {
			if self.validate_binary(entry).await.is_ok() {
				return Ok(entry.path.clone());
			}
		}

		drop(lock);

		let binary_path = self
			.install_game(game)
			.await
			.map_err(|e| format!("Failed to ensure binary: {e}"))?;

		Ok(binary_path)
	}

	/// Returns a list of installed games.
	pub async fn get_installed(&self) -> Result<Vec<Game>, String> {
		let _lock = self.lockfile_mutex.lock().await;
		let lockfile = self.load_lockfile().await?;

		let games = lockfile
			.binaries
			.values()
			.map(|entry| entry.game.clone())
			.collect();

		Ok(games)
	}

	/// Internal: Get the binary directory for a given game.
	fn binary_dir(&self, game: &Game) -> PathBuf {
		PathBuf::from(&self.binaries_dir)
			.join(game.identifier())
			.join(game.version_string())
	}

	/// Download a file from a URL.
	pub async fn download_file(
		reqwest_client: reqwest::Client,
		url: &Url,
		path: PathBuf,
	) -> Result<(), String> {
		let response = reqwest_client
			.get(url.clone())
			.send()
			.await
			.map_err(|e| format!("Failed to download: {e}"))?;

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
			.map_err(|e| format!("Failed to read response: {e}"))?;

		std::fs::write(path, bytes).map_err(|e| format!("Failed to save file: {e}"))?;

		Ok(())
	}

	/// Internal: Load the binary lockfile.
	async fn load_lockfile(&self) -> Result<BinaryLockfile, String> {
		let lockfile_path = PathBuf::from(format!("{}/binary.lock", &self.binaries_dir));

		if !lockfile_path.exists() {
			return Ok(BinaryLockfile::default());
		}

		let file_content = tokio::fs::read_to_string(lockfile_path)
			.await
			.map_err(|e| e.to_string())?;

		let lockfile: BinaryLockfile = toml::from_str(&file_content).map_err(|e| e.to_string())?;

		Ok(lockfile)
	}

	/// Internal: Save the binary lockfile.
	async fn save_lockfile(&self, lockfile: &BinaryLockfile) -> Result<(), String> {
		let lockfile_path = PathBuf::from(format!("{}/binary.lock", &self.binaries_dir));

		let file_content = toml::to_string(lockfile).map_err(|e| e.to_string())?;

		tokio::fs::write(lockfile_path, file_content)
			.await
			.map_err(|e| e.to_string())?;

		Ok(())
	}

	/// Internal: Validate a binary entry against its expected hash.
	async fn validate_binary(&self, entry: &BinaryLockfileEntry) -> Result<(), String> {
		if !entry.path.exists() {
			return Err(format!(
				"Binary path does not exist: {}",
				entry.path.display()
			));
		}

		if entry.hash.is_none() {
			return Ok(()); // No hash to validate against
		}

		// Clone because async block requires ownership
		let entry_path = entry.path.clone();
		let hash_info = entry.hash.clone().unwrap();

		let result = tokio::task::spawn_blocking(move || {
			compute_file_hash(hash_info.algorithm, &entry_path)
		})
		.await
		.map_err(|e| e.to_string())?;

		let file_hash = result?;

		if file_hash != hash_info.hash {
			return Err(format!(
				"Binary hash mismatch for {}: expected {}, got {}",
				entry.path.display(),
				hash_info.hash,
				file_hash
			));
		}

		Ok(())
	}
}
