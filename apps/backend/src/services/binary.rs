use crate::core::api_clients::fabric::FabricMetaAPIClient;
use crate::core::api_clients::paper::PaperMetaAPIClient;
use crate::core::bin_providers::paper::PaperBinaryProvider;
use crate::core::bin_providers::{
	fabric::FabricBinaryProvider, mojang_java::MojangJavaBinaryProvider, BinaryProvider,
};
use crate::core::files::binaries_lockfile::{
	BinaryLockfile, BinaryLockfileEntry, BinaryLockfileHash,
};
use crate::core::game::Game;
use crate::services::Service;
use crate::util::download::download_file;
use crate::util::hash::compute_file_hash;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BinaryService {
	binaries_dir: String,
	fabric: FabricBinaryProvider,
	mcje: MojangJavaBinaryProvider,
	paper: PaperBinaryProvider,
	lockfile_mutex: Arc<Mutex<()>>,
	reqwest_client: reqwest::Client,
}

impl Service for BinaryService {}

/// Service for managing game binaries.
impl BinaryService {
	pub fn new(reqwest_client: reqwest::Client) -> Self {
		let fabric_api = FabricMetaAPIClient::new(reqwest_client.clone());
		let paper_api = PaperMetaAPIClient::new(reqwest_client.clone());

		Self {
			binaries_dir: format!("{}/games", crate::config::DATA_FOLDER),
			fabric: FabricBinaryProvider::new(fabric_api),
			mcje: MojangJavaBinaryProvider::new(reqwest_client.clone()),
			paper: PaperBinaryProvider::new(paper_api),
			lockfile_mutex: Arc::new(Mutex::new(())),
			reqwest_client,
		}
	}

	/// Get the appropriate binary provider for a given game
	pub fn get_provider(&self, game: &Game) -> &dyn BinaryProvider {
		match game {
			Game::MinecraftJava { .. } => &self.mcje,
			Game::MinecraftJavaFabric { .. } => &self.fabric,
			Game::MinecraftJavaPaper { .. } => &self.paper,
		}
	}

	/// Installs a game with the specified configuration.
	pub async fn install_game(&self, game: Game) -> Result<PathBuf, String> {
		let _lock = self.lockfile_mutex.lock().await;
		let binary_dir = self.binary_dir(&game);

		// Ensure the binary directory exists
		if !binary_dir.exists() {
			std::fs::create_dir_all(&binary_dir)
				.map_err(|e| format!("Failed to create binary directory: {}", e))?;
		}

		let provider = self.get_provider(&game);
		let binary = provider.get(game.version()).await?;
		let download_url = binary.download_url();
		let binary_name = provider.binary_name();
		let binary_path = binary_dir.join(binary_name);

		download_file(
			self.reqwest_client.clone(),
			download_url,
			binary_path.clone(),
		)
		.await
		.map_err(|e| format!("Failed to download game: {}", e))?;

		// Add binary to the lockfile
		let mut lockfile = self
			.load_lockfile()
			.await
			.map_err(|e| format!("Failed to load lockfile: {}", e))?;

		let mut lockfile_entry = BinaryLockfileEntry {
			game: game.clone(),
			path: binary_path.clone(),
			hash: None,
		};

		if let Some((hash, hash_algorithm)) = binary.hash() {
			lockfile_entry.hash = Some(BinaryLockfileHash {
				algorithm: hash_algorithm,
				hash: hash.to_string(),
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
			.install_game(game.clone())
			.await
			.map_err(|e| format!("Failed to ensure binary: {}", e))?;

		return Ok(binary_path);
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
			.join(game.version().identifier())
	}

	/// Internal: Load the binary lockfile.
	async fn load_lockfile(&self) -> Result<BinaryLockfile, String> {
		let lockfile_path = PathBuf::from(format!("{}/binary.lock", &self.binaries_dir));

		if !lockfile_path.exists() {
			return Err("Binary lockfile does not exist".to_string());
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
			return Err(format!("Binary path does not exist: {:?}", entry.path));
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
				"Binary hash mismatch for {:?}: expected {}, got {}",
				entry.path, hash_info.hash, file_hash
			));
		}

		Ok(())
	}
}
