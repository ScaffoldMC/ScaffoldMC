use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::game::Game;

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct PartialServer {
	pub id: Uuid,
	pub name: String,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateServerRequest {
	pub name: String,
	pub game: Game,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct ServerCommandRequest {
	pub command: String,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct ConsoleQueryParams {
	pub since: Option<u64>,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct FilesGetQueryParams {
	pub content: Option<String>,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum FilesPostType {
	File,
	Directory,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct FilesPostQueryParams {
	#[serde(rename = "type")]
	pub entry_type: FilesPostType,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum FilesPutOperation {
	Rename,
	Move,
	Write,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct FilesPutQueryParams {
	pub operation: FilesPutOperation,
	pub to: Option<String>,
}
