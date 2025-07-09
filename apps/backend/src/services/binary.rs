use reqwest::Url;

use crate::core::bin_providers::BinaryInfo;
use crate::core::bin_providers::{
	fabric::FabricBinaryProvider, mojang_java::MojangJavaBinaryProvider, BinaryProvider,
};
use crate::core::game::Game;
use std::path::PathBuf;

pub struct BinaryService {
	fabric: FabricBinaryProvider,
	mcje: MojangJavaBinaryProvider,
}

impl BinaryService {
	pub fn new() -> Self {
		Self {
			fabric: FabricBinaryProvider::new(),
			mcje: MojangJavaBinaryProvider::new(),
		}
	}

	pub async fn get_games(&self) -> Result<Vec<Game>, String> {
		todo!("Implement game retrieval logic");
	}

	pub async fn install_game(&self, game: Game) -> Result<(), String> {
		let binary_dir = Self::binary_dir(&game);

		// Ensure the binary directory exists
		if !binary_dir.exists() {
			std::fs::create_dir_all(&binary_dir)
				.map_err(|e| format!("Failed to create binary directory: {}", e))?;
		}

		match game {
			Game::MCJava { version } => {
				let binary = self.mcje.get(version).await?;
				let download_url = binary.download_url();
				let binary_name = self.mcje.binary_name();
				let binary_path = binary_dir.join(binary_name);

				Self::download_file(download_url, binary_path)
					.await
					.map_err(|e| format!("Failed to download game: {}", e))?;
			}
			Game::MCJavaFabric { version } => {
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

	pub async fn ensure_binary(&self, game: &Game) -> Result<PathBuf, String> {
		let binary_name = match game {
			Game::MCJava { .. } => self.mcje.binary_name(),
			Game::MCJavaFabric { .. } => self.fabric.binary_name(),
		};

		let binary_path = Self::binary_dir(game).join(binary_name);

		if !binary_path.exists() {
			self.install_game(game.clone())
				.await
				.map_err(|e| format!("Failed to ensure binary: {}", e))?;
		}

		Ok(binary_path)
	}

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
				// TODO: Figure out how to load the game from the directory
				// if let Ok(game) = Game::from_path(&path) {
				// 	games.push(game);
				// }
			}
		}

		Ok(games)
	}

	fn binary_dir(game: &Game) -> PathBuf {
		PathBuf::from("data/games/")
			.join(game.identifier())
			.join(game.version().identifier())
	}

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
