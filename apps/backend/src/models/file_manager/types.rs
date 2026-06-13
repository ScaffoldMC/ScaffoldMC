pub struct FSDirectoryEntry {
	pub path: String,
	pub entries: Vec<FSFileEntry>,
}

pub struct FSFileEntry {
	pub path: String,
	pub content: Vec<u8>,
}

pub enum FSEntry {
	File(FSFileEntry),
	Dir(FSDirectoryEntry),
}

pub enum ScopedFileManagerError {
	PermissionDenied,
	IoError(std::io::Error),
}
