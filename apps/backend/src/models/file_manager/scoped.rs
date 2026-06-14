use crate::models::file_manager::types::ScopedFileManagerError::PermissionDenied;
use crate::models::file_manager::types::{
	FSDirectoryEntry, FSEntry, FSFileEntry, ScopedFileManagerError,
};
use crate::models::file_manager::FileManager;
use async_trait::async_trait;
use std::fs::create_dir;
use std::path::PathBuf;
use tokio::fs::{read_dir, remove_file, File};
use tokio::io::{BufReader, BufWriter};

pub struct ScopedFileManager {
	base_path: PathBuf,
}

impl ScopedFileManager {
	pub fn new(base_path: PathBuf) -> Self {
		Self { base_path }
	}

	/// Ensure the provided path is under base_path to prevent illegal paths
	pub fn check_path(&self, path: &PathBuf) -> Result<PathBuf, ScopedFileManagerError> {
		let joined = self.base_path.join(path);
		let canon_path = joined
			.canonicalize()
			.map_err(|err| ScopedFileManagerError::IoError(err))?;

		if canon_path.starts_with(&self.base_path) {
			Ok(canon_path)
		} else {
			Err(PermissionDenied)
		}
	}
}

#[async_trait]
impl FileManager for ScopedFileManager {
	type FileManagerError = ScopedFileManagerError;

	async fn read_file(&self, path: &PathBuf) -> Result<BufReader<File>, ScopedFileManagerError> {
		let path = self.check_path(path)?;

		let file = File::open(path)
			.await
			.map_err(|err| ScopedFileManagerError::IoError(err))?;

		let buf_reader = BufReader::new(file);

		Ok(buf_reader)
	}

	async fn write_file(&self, path: &PathBuf) -> Result<BufWriter<File>, ScopedFileManagerError> {
		let path = self.check_path(path)?;

		let file = File::open(path)
			.await
			.map_err(|err| ScopedFileManagerError::IoError(err))?;

		let buf_writer = BufWriter::new(file);

		Ok(buf_writer)
	}

	async fn delete(&self, path: &PathBuf) -> Result<(), ScopedFileManagerError> {
		let path = self.check_path(path)?;

		remove_file(path)
			.await
			.map_err(|err| ScopedFileManagerError::IoError(err))?;

		Ok(())
	}

	async fn create_dir(&self, path: &PathBuf) -> Result<(), ScopedFileManagerError> {
		let path = self.check_path(path)?;

		create_dir(path).map_err(|err| ScopedFileManagerError::IoError(err))?;

		Ok(())
	}

	async fn list_dir(&self, path: &PathBuf) -> Result<Vec<FSEntry>, ScopedFileManagerError> {
		let path = self.check_path(path)?;

		let mut dir = read_dir(path)
			.await
			.map_err(|err| ScopedFileManagerError::IoError(err))?;

		let mut entries = Vec::new();

		while let Some(entry) = dir
			.next_entry()
			.await
			.map_err(|err| ScopedFileManagerError::IoError(err))?
		{
			let file_type = entry
				.file_type()
				.await
				.map_err(|err| ScopedFileManagerError::IoError(err))?;

			if file_type.is_file() {
				entries.push(FSEntry::File(FSFileEntry {
					name: entry.file_name(),
				}));
			} else if file_type.is_dir() {
				entries.push(FSEntry::Dir(FSDirectoryEntry {
					name: entry.file_name(),
				}));
			}
		}

		Ok(entries)
	}

	async fn relocate(&self, path: &PathBuf) -> Result<(), ScopedFileManagerError> {
		let path = self.check_path(path)?;

		todo!()
	}

	async fn stat(&self, path: &PathBuf) -> Result<Vec<FSEntry>, ScopedFileManagerError> {
		let path = self.check_path(path)?;

		todo!()
	}
}
