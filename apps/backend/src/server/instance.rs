use std::path::PathBuf;

use tokio::process::{Child, Command};
use uuid::Uuid;

use crate::{config::SERVER_CONFIG_FILE_NAME, server::config::ServerConfig};
use thiserror::Error;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Error)]
pub enum ServerError {
	#[error("Server is already running")]
	AlreadyRunning,
	#[error("Failed to start server: {0}")]
	StartError(String),
	#[error("Failed to stop server: {0}")]
	StopError(String),
	#[error("Failed to send command to server: {0}")]
	CommandError(String),
	#[error("Server is not running")]
	NotRunning,
}

pub struct ServerInstance {
	id: Uuid,
	pub config: ServerConfig,
	process: Option<Child>,
}

impl ServerInstance {
	pub fn new(config: ServerConfig) -> Self {
		Self {
			id: Uuid::new_v4(),
			config,
			process: None,
		}
	}

	pub async fn send_command(&mut self, command: &str) -> Result<(), ServerError> {
		let child = self.process.as_mut().ok_or(ServerError::NotRunning)?;
		let stdin = child.stdin.as_mut().ok_or(ServerError::NotRunning)?;

		let command_with_newline = format!("{}\n", command);

		if let Err(err) = stdin.write_all(command_with_newline.as_bytes()).await {
			return Err(ServerError::CommandError(err.to_string()));
		}

		if let Err(err) = stdin.flush().await {
			return Err(ServerError::CommandError(err.to_string()));
		}

		Ok(())
	}

	pub async fn start(&mut self) -> Result<(), ServerError> {
		if self.process.is_some() {
			return Err(ServerError::AlreadyRunning);
		}

		let mut cmd = Command::new("java");

		for arg in &self.config.jvm_args {
			cmd.arg(arg);
		}

		cmd.arg("-jar");
		cmd.arg("server.jar"); // TODO: Get JAR path from version manager

		// TODO: Set current dir to server directory
		//cmd.current_dir(...);
		cmd.stdin(std::process::Stdio::piped());
		cmd.stdout(std::process::Stdio::piped());
		cmd.stderr(std::process::Stdio::piped());

		match cmd.spawn() {
			Ok(child) => {
				self.process = Some(child);
				Ok(())
			}
			Err(e) => Err(ServerError::StartError(e.to_string())),
		}
	}

	pub fn load_from_dir(path: PathBuf) -> Result<Self, String> {
		if !path.is_dir() {
			return Err(format!("Path {:?} is not a directory", path));
		}

		let config_path = path.join(SERVER_CONFIG_FILE_NAME);
		let server_config = ServerConfig::load_from_file(config_path);

		match server_config {
			Ok(server_config) => Ok(Self::new(server_config)),
			Err(e) => Err(format!("Failed to load server config: {}", e)),
		}
	}
}
