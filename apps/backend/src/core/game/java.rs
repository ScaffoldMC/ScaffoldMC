use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum JavaRuntime {
	Vanilla,
	Fabric { loader: String, launcher: String },
	Paper { build: u16 },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinecraftJava {
	pub version: String,
	pub loader: JavaRuntime,
}
