use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
	pub username: String,
	pub password: String,
}
