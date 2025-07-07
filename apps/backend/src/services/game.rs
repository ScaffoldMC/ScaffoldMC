use crate::util::mojang_api::get_version_info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Game {
	MCJava {
		version: String,
	},
	MCJavaFabric {
		mc_version: String,
		fabric_version: String,
	},
}

impl Game {
	pub fn identifier(&self) -> &str {
		match self {
			Game::MCJava { version } => "minecraft-java",
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => "minecraft-java-fabric",
		}
	}

	/// Get the download URL for this game
	pub async fn get_download_url(&self) -> String {
		match self {
			Game::MCJava { version } => get_version_info(version)
				.await
				.expect("Failed to get version info") // TODO: Handle error properly
				.url
				.clone(),
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => {
				format!("{FABRIC_API_URL}/versions/loader/{mc_version}/{fabric_version}/server/jar")
			}
		}
	}

	/// Get the name of the binary for this game
	pub fn binary_name(&self) -> &str {
		match self {
			Game::MCJava { version } => "server.jar",
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => "server.jar",
		}
	}

	/// Get the version of this game as a single string
	pub fn version(&self) -> String {
		match self {
			Game::MCJava { version } => version.clone(),
			Game::MCJavaFabric {
				mc_version,
				fabric_version,
			} => format!("{mc_version}-{fabric_version}"),
		}
	}
}

pub struct GameService {}

impl GameService {
	pub fn new() -> Self {
		Self {}
	}

	pub async fn get_games(&self) -> Result<Vec<Game>, String> {
		todo!("Implement game retrieval logic");
	}

	pub async fn install_game(&self, game: Game) -> Result<(), String> {
		let download_url = game.get_download_url().await;
		let binary_dir = Self::binary_dir(&game);

		// Ensure the binary directory exists
		if !binary_dir.exists() {
			std::fs::create_dir_all(&binary_dir)
				.map_err(|e| format!("Failed to create binary directory: {}", e))?;
		}

		let binary_path = Self::binary_dir(&game).join(game.binary_name());

		// TODO: Might want to spawn blocking?

		Self::download_file(&download_url, binary_path)
			.await
			.map_err(|e| format!("Failed to download game: {}", e))?;

		Ok(())
	}

	pub async fn ensure_binary(&self, game: &Game) -> Result<PathBuf, String> {
		let binary_path = Self::binary_dir(game).join(game.binary_name());

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
			.join(game.version())
	}

	async fn download_file(url: &str, path: PathBuf) -> Result<(), String> {
		let response = reqwest::get(url)
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
