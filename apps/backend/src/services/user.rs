use std::sync::Arc;

use crate::{
	db::{user::User, Database},
	services::Service,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
	#[error("Username already taken")]
	UsernameTaken,
	#[error("Internal server error: {0}")]
	ServerError(String),
}

pub struct UserService {
	db: Arc<Database>,
}

impl Service for UserService {}

impl UserService {
	pub fn new(db: Arc<Database>) -> Self {
		Self { db }
	}

	/// Change a user's username.
	pub async fn change_username(
		&self,
		user: &User,
		new_username: &str,
	) -> Result<(), UserServiceError> {
		if let Ok(_) = self.db.get_user_by_username(new_username).await {
			return Err(UserServiceError::UsernameTaken);
		}

		if let Err(err) = self.db.update_user_username(user.id, new_username).await {
			return Err(UserServiceError::ServerError(err.to_string()));
		}

		Ok(())
	}
}
