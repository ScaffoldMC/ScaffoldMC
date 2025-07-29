use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum HashAlgorithm {
	Sha256,
}

pub fn compute_file_hash(alg: HashAlgorithm, path: &PathBuf) -> Result<String, String> {
	let mut hasher = match alg {
		HashAlgorithm::Sha256 => Sha256::new(),
	};

	let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
	let mut reader = std::io::BufReader::new(file);

	std::io::copy(&mut reader, &mut hasher)
		.map_err(|e| format!("Failed to read file for hashing: {}", e))?;

	let hash = hasher.finalize();

	Ok(format!("{:x}", hash))
}
