use digest::DynDigest;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::Sha256;

use std::{io::Write, path::PathBuf};

// Make DynDigest allow std::io::copy
// https://github.com/RustCrypto/hashes/issues/430#issuecomment-1304584228
trait WritableDynDigest: DynDigest + Write {}
impl<T: DynDigest + Write> WritableDynDigest for T {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum HashAlgorithm {
	Sha256,
	Sha1,
}

pub fn compute_file_hash(alg: HashAlgorithm, path: &PathBuf) -> Result<String, String> {
	let mut hasher_box: Box<dyn WritableDynDigest> = match alg {
		HashAlgorithm::Sha256 => Box::new(Sha256::default()),
		HashAlgorithm::Sha1 => Box::new(Sha1::default()),
	};

	let mut hasher = &mut *hasher_box;

	let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
	let mut reader = std::io::BufReader::new(file);

	std::io::copy(&mut reader, &mut hasher)
		.map_err(|e| format!("Failed to read file for hashing: {e}"))?;

	let hash = hasher.finalize_reset();

	Ok(hex::encode(&*hash))
}
