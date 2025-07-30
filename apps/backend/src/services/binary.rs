use crate::core::bin_providers::{
	fabric::FabricBinaryProvider, mojang_java::MojangJavaBinaryProvider, BinaryProvider,
};
use crate::core::game::Game;
use crate::services::Service;
use crate::util::download::download_file;
use crate::util::hash::{compute_file_hash, HashAlgorithm};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryLockfile {
	version: String,
	binaries: HashMap<String, BinaryLockfileEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryLockfileEntry {
	game: Game,
	path: PathBuf,
	hash: String,
	hash_algorithm: HashAlgorithm,
}

pub struct BinaryService {
	binaries_dir: String,
	fabric: FabricBinaryProvider,
	mcje: MojangJavaBinaryProvider,
	lockfile_mutex: Arc<Mutex<()>>,
}

impl Service for BinaryService {}

/// Service for managing game binaries.
impl BinaryService {
	pub fn new() -> Self {
		Self {
			binaries_dir: format!("{}/games", crate::config::DATA_FOLDER),
			fabric: FabricBinaryProvider::new(),
			mcje: MojangJavaBinaryProvider::new(),
			lockfile_mutex: Arc::new(Mutex::new(())),
		}
	}

	/// Get the appropriate binary provider for a given game
	pub fn get_provider(&self, game: &Game) -> &dyn BinaryProvider {
		match game {
			Game::MinecraftJava { .. } => &self.mcje,
			Game::MinecraftJavaFabric { .. } => &self.fabric,
		}
	}

	/// Retrieves a list of available games.
	pub async fn get_games(&self) -> Result<Vec<Game>, String> {
		todo!("Retrieve available games from the provider");
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

		download_file(download_url, binary_path.clone())
			.await
			.map_err(|e| format!("Failed to download game: {}", e))?;

		// Add binary to the lockfile
		let mut lockfile = self
			.load_lockfile()
			.await
			.map_err(|e| format!("Failed to load lockfile: {}", e))?;

		let lockfile_entry = BinaryLockfileEntry {
			game: game.clone(),
			path: binary_path.clone(),
			hash: String::new(), // TODO: Get hash
			hash_algorithm: HashAlgorithm::Sha256,
		};

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

	async fn save_lockfile(&self, lockfile: &BinaryLockfile) -> Result<(), String> {
		let lockfile_path = PathBuf::from(format!("{}/binary.lock", &self.binaries_dir));

		let file_content = toml::to_string(lockfile).map_err(|e| e.to_string())?;

		tokio::fs::write(lockfile_path, file_content)
			.await
			.map_err(|e| e.to_string())?;

		Ok(())
	}

	async fn validate_binary(&self, entry: &BinaryLockfileEntry) -> Result<(), String> {
		if !entry.path.exists() {
			return Err(format!("Binary path does not exist: {:?}", entry.path));
		}

		let entry_path = entry.path.clone();
		let algorithm = entry.hash_algorithm;

		let result = tokio::task::spawn_blocking(move || compute_file_hash(algorithm, &entry_path))
			.await
			.map_err(|e| e.to_string())?;

		let file_hash = result?;

		if file_hash != entry.hash {
			return Err(format!(
				"Binary hash mismatch for {:?}: expected {}, got {}",
				entry.path, entry.hash, file_hash
			));
		}

		Ok(())
	}
}
