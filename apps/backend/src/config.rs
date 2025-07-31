use time::Duration;

// File system
pub const DATA_FOLDER: &str = "data";
pub const SERVER_CONFIG_FILE_NAME: &str = "server_config.json";

// Authentication
pub static REFRESH_COOKIE_NAME: &str = "ref_token";
pub static AUTH_COOKIE_NAME: &str = "auth_token";
pub static REFRESH_TOKEN_LENGTH: Duration = Duration::hours(6);
pub static AUTH_TOKEN_LENGTH: Duration = Duration::minutes(5);

// Reqwest
pub static CLIENT_USER_AGENT: &str =
	"MCServerUI/1.0.0 (https://github.com/unwieldycat/mc-server-ui)";
