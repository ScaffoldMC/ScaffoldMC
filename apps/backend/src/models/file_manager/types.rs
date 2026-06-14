use std::ffi::OsString;

pub struct FSDirectoryEntry {
	pub name: OsString,
}

pub struct FSFileEntry {
	pub name: OsString,
	pub size: u64,
}

pub enum FSEntry {
	File(FSFileEntry),
	Dir(FSDirectoryEntry),
}

pub enum FileManagerError {
	PermissionDenied,
	UnknownType,
	IoError(std::io::Error),
}
