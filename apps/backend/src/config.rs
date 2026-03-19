use time::Duration;
use tokio::time::Duration as TokioDuration;

// File system
pub const DATA_FOLDER: &str = "data";
pub const SERVER_CONFIG_FILE_NAME: &str = "server_config.toml";

// Authentication
pub static REFRESH_COOKIE_NAME: &str = "ref_token";
pub static AUTH_COOKIE_NAME: &str = "auth_token";
pub static REFRESH_TOKEN_LENGTH: Duration = Duration::hours(6);
pub static AUTH_TOKEN_LENGTH: Duration = Duration::minutes(5);

// Reqwest
pub static CLIENT_USER_AGENT: &str = "ScaffoldMC/0.0.0 (https://github.com/ScaffoldMC/ScaffoldMC)";

// Server runtime
pub static SERVER_WATCHER_TICK: TokioDuration = TokioDuration::from_millis(200);
pub static SERVER_CONSOLE_MAX_LINES: usize = 500;
