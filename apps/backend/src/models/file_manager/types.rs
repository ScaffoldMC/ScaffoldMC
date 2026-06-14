use std::ffi::OsString;

pub struct FSDirectoryEntry {
	pub name: OsString,
}

pub struct FSFileEntry {
	pub name: OsString,
}

pub enum FSEntry {
	File(FSFileEntry),
	Dir(FSDirectoryEntry),
}

pub enum ScopedFileManagerError {
	PermissionDenied,
	IoError(std::io::Error),
}
