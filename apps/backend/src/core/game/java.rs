use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Clone, Deserialize, Serialize)]
#[ts(export)]
#[serde(rename_all = "snake_case")]
pub enum MinecraftJavaLoader {
	Vanilla,
	Fabric { loader: String, launcher: String },
	Paper { build: u16 },
}

#[derive(TS, Debug, Clone, Deserialize, Serialize)]
#[ts(export)]
pub struct MinecraftJava {
	pub version: String,
	pub loader: MinecraftJavaLoader,
}
