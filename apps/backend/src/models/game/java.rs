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

impl MinecraftJavaLoader {
	pub fn version_string(&self) -> String {
		match self {
			MinecraftJavaLoader::Vanilla => "vanilla".to_string(),
			MinecraftJavaLoader::Fabric { loader, launcher } => {
				format!("fabric-{loader}-{launcher}")
			}
			MinecraftJavaLoader::Paper { build } => format!("paper-{build}"),
		}
	}
}

#[derive(TS, Debug, Clone, Deserialize, Serialize)]
#[ts(export)]
pub struct MinecraftJava {
	pub version: String,
	pub loader: MinecraftJavaLoader,
}
