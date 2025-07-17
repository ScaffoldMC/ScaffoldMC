use reqwest::Url;

use crate::core::bin_providers::BinaryInfo;
use crate::core::bin_providers::{
	fabric::FabricBinaryProvider, mojang_java::MojangJavaBinaryProvider, BinaryProvider,
};
use crate::core::game::{self, Game};
use std::path::PathBuf;

pub struct BinaryService {
	fabric: FabricBinaryProvider,
	mcje: MojangJavaBinaryProvider,
}

/// Service for managing game binaries.
impl BinaryService {
	pub fn new() -> Self {
		Self {
			fabric: FabricBinaryProvider::new(),
			mcje: MojangJavaBinaryProvider::new(),
		}
	}

	/// Retrieves a list of available games.
	pub async fn get_games(&self) -> Result<Vec<Game>, String> {
		todo!("Implement game retrieval logic");
	}

	/// Installs a game with the specified configuration.
	pub async fn install_game(&self, game: Game) -> Result<(), String> {
		let binary_dir = Self::binary_dir(&game);

		// Ensure the binary directory exists
		if !binary_dir.exists() {
			std::fs::create_dir_all(&binary_dir)
				.map_err(|e| format!("Failed to create binary directory: {}", e))?;
		}

		match game {
			Game::MinecraftJava { version } => {
				let binary = self.mcje.get(version).await?;
				let download_url = binary.download_url();
				let binary_name = self.mcje.binary_name();
				let binary_path = binary_dir.join(binary_name);

				Self::download_file(download_url, binary_path)
					.await
					.map_err(|e| format!("Failed to download game: {}", e))?;
			}
			Game::MinecraftJavaFabric { version } => {
				let binary = self.fabric.get(version).await?;
				let download_url = binary.download_url();
				let binary_name = self.fabric.binary_name();
				let binary_path = binary_dir.join(binary_name);

				Self::download_file(download_url, binary_path)
					.await
					.map_err(|e| format!("Failed to download game: {}", e))?;
			}
		};

		Ok(())
	}

	/// Returns the path to the binary directory for a given game, if not
	/// available it will be downloaded.
	pub async fn ensure_binary(&self, game: &Game) -> Result<PathBuf, String> {
		let binary_name = match game {
			Game::MinecraftJava { .. } => self.mcje.binary_name(),
			Game::MinecraftJavaFabric { .. } => self.fabric.binary_name(),
		};

		let binary_path = Self::binary_dir(game).join(binary_name);

		if !binary_path.exists() {
			self.install_game(game.clone())
				.await
				.map_err(|e| format!("Failed to ensure binary: {}", e))?;
		}

		Ok(binary_path)
	}

	/// Returns a list of installed games.
	pub async fn get_installed(&self) -> Result<Vec<Game>, String> {
		let games_dir = PathBuf::from("data/games/");
		if !games_dir.exists() {
			return Ok(vec![]);
		}

		let mut games = Vec::new();

		for entry in std::fs::read_dir(games_dir).map_err(|e| e.to_string())? {
			let entry = entry.map_err(|e| e.to_string())?;
			let path = entry.path();

			if path.is_dir() {
				continue;
			}

			let game_name = path
				.file_name()
				.ok_or("Failed to get dir name from subentry")?
				.to_str()
				.ok_or("Failed to transform dir name to str")?;

			for subentry in std::fs::read_dir(&path).map_err(|e| e.to_string())? {
				let subentry = subentry.map_err(|e| e.to_string())?;
				let subpath = subentry.path();

				if !subpath.is_dir() {
					continue;
				}

				let ver_name = subpath
					.file_name()
					.ok_or("Failed to get dir name from subentry")?
					.to_str()
					.ok_or("Failed to transform dir name to str")?;

				let game = Game::from_path_parts(game_name, ver_name)?;

				games.push(game);
			}
		}

		Ok(games)
	}

	/// Internal: Get the binary directory for a given game.
	fn binary_dir(game: &Game) -> PathBuf {
		PathBuf::from("data/games/")
			.join(game.identifier())
			.join(game.version().identifier())
	}

	/// Internal: Download a file from a URL.
	async fn download_file(url: &Url, path: PathBuf) -> Result<(), String> {
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
}
