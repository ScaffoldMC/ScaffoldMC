use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export)]
pub struct FSDirectoryEntry {
	pub name: String,
}

#[derive(TS, Debug, Serialize)]
#[ts(export)]
pub struct FSFileEntry {
	pub name: String,
	pub size: u64,
}

#[derive(TS, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FSEntry {
	File(FSFileEntry),
	Dir(FSDirectoryEntry),
}

#[derive(Debug, Error)]
pub enum FileManagerError {
	#[error("Path is invalid or outside of allowed directory")]
	InvalidPath,
	#[error("Unknown file type")]
	UnknownType,
	#[error("Non UTF-8 string encountered in file name")]
	EncodingError,
	#[error("I/O error: {0}")]
	IoError(std::io::Error),
}
