use std::{path::PathBuf, sync::LazyLock};

use time::Duration;
use tokio::time::Duration as TokioDuration;
use uuid::Uuid;

// File system
pub const DATA_FOLDER: &str = "data";
pub const SERVER_CONFIG_FILE_NAME: &str = "server_config.toml";

// Authentication
pub static REFRESH_COOKIE_NAME: &str = "ref_token";
pub static AUTH_COOKIE_NAME: &str = "auth_token";
pub static REFRESH_TOKEN_LENGTH: Duration = Duration::hours(6);
pub static AUTH_TOKEN_LENGTH: Duration = Duration::minutes(5);
pub static RSA_KEY_SIZE: usize = 3072;

// Reqwest
pub static CLIENT_USER_AGENT: &str = "ScaffoldMC/0.0.0 (https://github.com/ScaffoldMC/ScaffoldMC)";

// Server runtime
pub static SERVER_WATCHER_TICK: TokioDuration = TokioDuration::from_millis(200);
pub static SERVER_CONSOLE_MAX_LINES: usize = 500;

// APIs
pub static FABRIC_API_URL: &str = "https://meta.fabricmc.net/v2";
pub static PAPER_API_URL: &str = "https://fill.papermc.io/v3/projects/paper";
pub static MOJANG_API_URL: &str = "https://piston-meta.mojang.com";

// Generated variables

pub static SERVERS_DIRECTORY: LazyLock<String> = LazyLock::new(|| format!("{DATA_FOLDER}/servers"));

// Helper functions

/// Get the canonical directory for a server
pub fn canonical_server_dir(server_id: Uuid) -> PathBuf {
	format!("{}/{}", SERVERS_DIRECTORY.clone(), server_id).into()
}
