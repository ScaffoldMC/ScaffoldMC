use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export)]
pub struct PartialServer {
	pub id: Uuid,
	pub name: String,
}
