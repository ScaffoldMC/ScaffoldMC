use async_trait::async_trait;
use std::path::PathBuf;

pub struct VFSDirectoryEntry {
	pub path: String,
	pub entries: Vec<VFSFileEntry>,
}

pub struct VFSFileEntry {
	pub path: String,
	pub content: Vec<u8>,
}

pub enum VFSEntry {
	File(VFSFileEntry),
	Dir(VFSDirectoryEntry),
}

pub enum FileManagerError {
	NotFound,
	PermissionDenied,
	InvalidPath,
	IoError(std::io::Error),
}

#[async_trait]
pub trait FileManager: Send + Sync {
	/// Read a file
	async fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>, FileManagerError>;

	/// Write a file
	async fn write_file(&self, path: &PathBuf, content: &[u8]) -> Result<(), FileManagerError>;

	/// Delete a file or directory
	async fn delete(&self, path: &PathBuf) -> Result<(), FileManagerError>;

	/// Create a directory
	async fn create_dir(&self, path: &PathBuf) -> Result<(), FileManagerError>;

	/// List the contents of a directory
	async fn list_dir(&self, path: &PathBuf) -> Result<Vec<VFSEntry>, FileManagerError>;

	/// Move a file or directory
	async fn relocate(&self, path: &PathBuf) -> Result<(), FileManagerError>;

	/// Get information about a file or directory.
	async fn stat(&self, path: &PathBuf) -> Result<Vec<VFSEntry>, FileManagerError>;
}

// TODO: Implement
pub struct VirtualFileManager {}
