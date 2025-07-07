use crate::util::mojang_api::get_version_info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MinecraftJavaLoader {
	Fabric { version: String },
	Paper { version: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Game {
	MinecraftJava {
		version: String,
		loader: Option<MinecraftJavaLoader>,
	},
}

impl Game {
	pub fn from_path(path: &PathBuf) -> Result<Self, String> {
		let mut components = path.components();
		todo!("Parse components to determine game type");
	}

	/// Get the download URL for this game
	pub async fn get_download_url(&self) -> String {
		match self {
			Game::MinecraftJava { version, loader } => match loader {
				Some(MinecraftJavaLoader::Fabric {
					version: loader_version,
				}) => format!(
					"{FABRIC_API_URL}/versions/loader/{version}/{loader_version}/server/jar"
				),
				Some(MinecraftJavaLoader::Paper {
					version: loader_version,
				}) => todo!(),
				None => get_version_info(version)
					.await
					.expect("Failed to get version info") // TODO: Handle error properly
					.url
					.clone(),
			},
		}
	}

	pub fn get_binary_path(&self) -> PathBuf {
		match self {
			Game::MinecraftJava { version, loader } => self.get_binary_dir().join("server.jar"),
		}
	}

	pub fn get_binary_dir(&self) -> PathBuf {
		match self {
			Game::MinecraftJava { version, loader } => match loader {
				Some(MinecraftJavaLoader::Fabric {
					version: loader_version,
				}) => PathBuf::from(format!("mcje/{}/fabric/{}/", version, loader_version)),
				Some(MinecraftJavaLoader::Paper {
					version: loader_version,
				}) => PathBuf::from(format!("mcje/{}/paper/{}/", version, loader_version)),
				None => PathBuf::from(format!("mcje/{}/vanilla/", version)),
			},
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
		let binary_dir = PathBuf::from("data/games/").join(game.get_binary_dir());

		// Ensure the binary directory exists
		if !binary_dir.exists() {
			std::fs::create_dir_all(&binary_dir)
				.map_err(|e| format!("Failed to create binary directory: {}", e))?;
		}

		let binary_path = PathBuf::from("data/games/").join(game.get_binary_path());

		// TODO: Might want to spawn blocking?

		Self::download_file(&download_url, binary_path)
			.await
			.map_err(|e| format!("Failed to download game: {}", e))?;

		Ok(())
	}

	pub async fn ensure_binary(&self, game: &Game) -> Result<PathBuf, String> {
		let binary_path = PathBuf::from("data/games/").join(game.get_binary_path());

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
				if let Ok(game) = Game::from_path(&path) {
					games.push(game);
				}
			}
		}

		Ok(games)
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
