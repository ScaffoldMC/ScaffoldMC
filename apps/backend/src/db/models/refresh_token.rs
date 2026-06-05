use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
	pub id: String,
	pub user_id: Uuid,
	pub created_at: OffsetDateTime,
}
