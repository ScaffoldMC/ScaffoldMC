use crate::models::file_manager::types::{FSEntry, FileManagerError};
use crate::models::file_manager::FileManager;
use async_trait::async_trait;
use std::fs::create_dir;
use std::path::PathBuf;
use tokio::fs::{remove_file, File};
use tokio::io::{BufReader, BufWriter};

pub struct ScopedFileManager {
	base_path: PathBuf,
}

impl ScopedFileManager {
	pub fn new(base_path: PathBuf) -> Self {
		Self { base_path }
	}
}

#[async_trait]
impl FileManager for ScopedFileManager {
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

	async fn list_dir(&self, _path: &PathBuf) -> Result<Vec<FSEntry>, FileManagerError> {
		// TODO: map read_dir result to struct
		todo!()
	}

	async fn relocate(&self, _path: &PathBuf) -> Result<(), FileManagerError> {
		todo!()
	}

	async fn stat(&self, _path: &PathBuf) -> Result<Vec<FSEntry>, FileManagerError> {
		todo!()
	}
}
