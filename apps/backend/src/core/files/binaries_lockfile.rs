use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{core::game::Game, util::hash::HashAlgorithm};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryLockfile {
	pub version: u8,
	pub binaries: HashMap<String, BinaryLockfileEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryLockfileHash {
	pub algorithm: HashAlgorithm,
	pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryLockfileEntry {
	pub game: Game,
	pub path: PathBuf,
	pub hash: Option<BinaryLockfileHash>,
}
