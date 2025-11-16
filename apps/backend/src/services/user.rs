use std::sync::Arc;

use crate::{
	db::{user::User, Database},
	services::Service,
};

use thiserror::Error;
use tokio::task::spawn_blocking;

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

	/// Retrieve a user by their ID.
	pub async fn get_user_by_id(&self, user_id: uuid::Uuid) -> Result<User, UserServiceError> {
		match self.db.get_user_by_id(user_id).await {
			Ok(user) => Ok(user),
			Err(err) => Err(UserServiceError::ServerError(err.to_string())),
		}
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

	/// Change a user's full name.
	pub async fn change_full_name(
		&self,
		user: &User,
		new_full_name: &str,
	) -> Result<(), UserServiceError> {
		if let Err(err) = self.db.update_user_fullname(user.id, new_full_name).await {
			return Err(UserServiceError::ServerError(err.to_string()));
		}

		Ok(())
	}

	/// Change a user's password.
	pub async fn change_password(
		&self,
		user: &User,
		new_password: &str,
	) -> Result<(), UserServiceError> {
		let password_owned = new_password.to_owned();
		let new_password_hash =
			spawn_blocking(move || password_auth::generate_hash(&password_owned)).await;

		if let Err(e) = new_password_hash {
			return Err(UserServiceError::ServerError(e.to_string()));
		}

		let new_password_hash = new_password_hash.unwrap();

		if let Err(err) = self
			.db
			.update_user_password_hash(user.id, &new_password_hash)
			.await
		{
			return Err(UserServiceError::ServerError(err.to_string()));
		}

		Ok(())
	}
}
