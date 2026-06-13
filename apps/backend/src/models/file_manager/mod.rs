use crate::models::file_manager::types::FSEntry;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{BufReader, BufWriter};

pub mod scoped;
pub mod types;

#[async_trait]
pub trait FileManager: Send + Sync {
	type FileManagerError;

	/// Get a read buffer to a file
	async fn read_file(&self, path: &PathBuf) -> Result<BufReader<File>, Self::FileManagerError>;

	/// Get a write buffer to a file
	async fn write_file(&self, path: &PathBuf) -> Result<BufWriter<File>, Self::FileManagerError>;

	/// Delete a file or directory
	async fn delete(&self, path: &PathBuf) -> Result<(), Self::FileManagerError>;

	/// Create a directory
	async fn create_dir(&self, path: &PathBuf) -> Result<(), Self::FileManagerError>;

	/// List the contents of a directory
	async fn list_dir(&self, path: &PathBuf) -> Result<Vec<FSEntry>, Self::FileManagerError>;

	/// Move a file or directory
	async fn relocate(&self, path: &PathBuf) -> Result<(), Self::FileManagerError>;

	/// Get information about a file or directory.
	async fn stat(&self, path: &PathBuf) -> Result<Vec<FSEntry>, Self::FileManagerError>;
}
