use async_trait::async_trait;
use std::fs::create_dir;
use std::path::PathBuf;
use tokio::fs::{remove_file, File};
use tokio::io::{BufReader, BufWriter};

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
	/// Create a new file manager with the given permissions
	fn new(base_path: PathBuf) -> Self
	where
		Self: Sized;

	/// Get a read buffer to a file
	async fn read_file(&self, path: &PathBuf) -> Result<BufReader<File>, FileManagerError>;

	/// Get a write buffer to a file
	async fn write_file(&self, path: &PathBuf) -> Result<BufWriter<File>, FileManagerError>;

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

pub struct VirtualFileManager {
	base_path: PathBuf,
}

#[async_trait]
impl FileManager for VirtualFileManager {
	fn new(base_path: PathBuf) -> Self {
		Self { base_path }
	}

	async fn read_file(&self, path: &PathBuf) -> Result<BufReader<File>, FileManagerError> {
		let file = File::open(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		let buf_reader = BufReader::new(file);

		Ok(buf_reader)
	}

	async fn write_file(&self, path: &PathBuf) -> Result<BufWriter<File>, FileManagerError> {
		let file = File::open(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		let buf_writer = BufWriter::new(file);

		Ok(buf_writer)
	}

	async fn delete(&self, path: &PathBuf) -> Result<(), FileManagerError> {
		remove_file(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		Ok(())
	}

	async fn create_dir(&self, path: &PathBuf) -> Result<(), FileManagerError> {
		create_dir(path).map_err(|err| FileManagerError::IoError(err))?;

		Ok(())
	}

	async fn list_dir(&self, _path: &PathBuf) -> Result<Vec<VFSEntry>, FileManagerError> {
		// TODO: map read_dir result to struct
		todo!()
	}

	async fn relocate(&self, _path: &PathBuf) -> Result<(), FileManagerError> {
		todo!()
	}

	async fn stat(&self, _path: &PathBuf) -> Result<Vec<VFSEntry>, FileManagerError> {
		todo!()
	}
}
