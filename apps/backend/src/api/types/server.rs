use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct PartialServer {
	pub id: Uuid,
	pub name: String,
}
