use crate::models::file_manager::types::{FSEntry, FileManagerError};
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{BufReader, BufWriter};

pub mod scoped;
pub mod types;

#[async_trait]
pub trait FileManager: Send + Sync {
	/// Get a read buffer to a file
	async fn read_file(&self, path: &Path) -> Result<BufReader<File>, FileManagerError>;

	/// Get a write buffer to a file
	async fn write_file(&self, path: &Path) -> Result<BufWriter<File>, FileManagerError>;

	/// Delete a file or directory
	async fn delete(&self, path: &Path) -> Result<(), FileManagerError>;

	/// Create a directory
	async fn create_dir(&self, path: &Path) -> Result<(), FileManagerError>;

	/// Create a file
	async fn create_file(&self, path: &Path) -> Result<(), FileManagerError>;

	/// List the contents of a directory
	async fn list_dir(&self, path: &Path) -> Result<Vec<FSEntry>, FileManagerError>;

	/// Move a file or directory
	async fn relocate(&self, path: &Path, new_path: &Path) -> Result<(), FileManagerError>;

	/// Get information about a file or directory.
	async fn stat(&self, path: &Path) -> Result<FSEntry, FileManagerError>;
}
