use crate::models::file_manager::types::FileManagerError::{IoError, NoPermission, NotFound};
use crate::models::file_manager::types::{
	FSDirectoryEntry, FSEntry, FSFileEntry, FileManagerError,
};
use crate::models::file_manager::FileManager;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::{create_dir, metadata, read_dir, remove_file, rename, File};
use tokio::io::{self, BufReader, BufWriter};

pub struct ScopedFileManager {
	base_path: PathBuf,
}

impl ScopedFileManager {
	pub fn new(base_path: PathBuf) -> Self {
		Self { base_path }
	}

	/// Ensure the provided path is under base_path to prevent illegal paths
	pub fn check_path(&self, path: &PathBuf) -> Result<PathBuf, FileManagerError> {
		let joined = self.base_path.join(path);
		let canon_path_result = joined.canonicalize();

		if let Err(err) = canon_path_result {
			if err.kind() == io::ErrorKind::NotFound {
				return Err(NotFound);
			}

			return Err(IoError(err));
		}

		let canon_path = canon_path_result.unwrap();

		tracing::info!("path: {:?}", canon_path);

		if canon_path.starts_with(&self.base_path) {
			Ok(canon_path)
		} else {
			Err(NoPermission)
		}
	}
}

#[async_trait]
impl FileManager for ScopedFileManager {
	async fn read_file(&self, path: &PathBuf) -> Result<BufReader<File>, FileManagerError> {
		let path = self.check_path(path)?;

		let file = File::open(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		let buf_reader = BufReader::new(file);

		Ok(buf_reader)
	}

	async fn write_file(&self, path: &PathBuf) -> Result<BufWriter<File>, FileManagerError> {
		let path = self.check_path(path)?;

		let file = File::open(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		let buf_writer = BufWriter::new(file);

		Ok(buf_writer)
	}

	async fn delete(&self, path: &PathBuf) -> Result<(), FileManagerError> {
		let path = self.check_path(path)?;

		remove_file(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		Ok(())
	}

	async fn create_dir(&self, path: &PathBuf) -> Result<(), FileManagerError> {
		let path = self.check_path(path)?;

		create_dir(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		Ok(())
	}

	async fn list_dir(&self, path: &PathBuf) -> Result<Vec<FSEntry>, FileManagerError> {
		let path = self.check_path(path)?;

		let mut dir = read_dir(path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		let mut entries = Vec::new();

		while let Some(entry) = dir
			.next_entry()
			.await
			.map_err(|err| FileManagerError::IoError(err))?
		{
			let file_type = entry
				.file_type()
				.await
				.map_err(|err| FileManagerError::IoError(err))?;

			let metadata = entry
				.metadata()
				.await
				.map_err(|err| FileManagerError::IoError(err))?;

			let name = entry
				.file_name()
				.into_string()
				.map_err(|_| FileManagerError::EncodingError)?;

			if file_type.is_file() {
				entries.push(FSEntry::File(FSFileEntry {
					name,
					size: metadata.len(),
				}));
			} else if file_type.is_dir() {
				entries.push(FSEntry::Dir(FSDirectoryEntry { name }));
			}
		}

		Ok(entries)
	}

	async fn relocate(&self, path: &PathBuf, new_path: &PathBuf) -> Result<(), FileManagerError> {
		let path = self.check_path(path)?;
		let new_path = self.check_path(new_path)?;

		rename(path, new_path)
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		Ok(())
	}

	async fn stat(&self, path: &PathBuf) -> Result<FSEntry, FileManagerError> {
		let path = self.check_path(path)?;

		let metadata = metadata(path.clone())
			.await
			.map_err(|err| FileManagerError::IoError(err))?;

		let name = path
			.file_name()
			.ok_or(FileManagerError::NoPermission)?
			.to_owned()
			.into_string()
			.map_err(|_| FileManagerError::EncodingError)?;

		if metadata.is_file() {
			Ok(FSEntry::File(FSFileEntry {
				name,
				size: metadata.len(),
			}))
		} else if metadata.is_dir() {
			Ok(FSEntry::Dir(FSDirectoryEntry { name }))
		} else {
			Err(FileManagerError::UnknownType)
		}
	}
}
