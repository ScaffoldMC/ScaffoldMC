use std::{
	collections::VecDeque,
	sync::{atomic::AtomicU64, Arc},
};

use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch, RwLock};
use ts_rs::TS;
use uuid::Uuid;

use crate::core::files::server_config::ServerConfig;

#[derive(Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct ConsoleLine {
	pub num: u64,
	pub stream: ConsoleStreamType,
	pub line: String,
}

#[derive(Clone, Copy, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum ConsoleStreamType {
	Stdout,
	Stderr,
}

#[derive(Debug, Clone)]
pub enum ProcessCommand {
	Kill,
	Write(String),
}

pub struct ServerRuntime {
	pub command_tx: mpsc::Sender<ProcessCommand>,
	pub running_rx: watch::Receiver<bool>,
}

impl ServerRuntime {
	pub async fn kill(&self) -> Result<(), String> {
		self.command_tx
			.send(ProcessCommand::Kill)
			.await
			.map_err(|e| format!("Failed to send kill command: {e}"))?;

		Ok(())
	}

	pub async fn send_line(&self, line: String) -> Result<(), String> {
		self.command_tx
			.send(ProcessCommand::Write(line))
			.await
			.map_err(|_| "Process supervisor is not running".to_string())
	}

	pub fn is_running(&self) -> bool {
		*self.running_rx.borrow()
	}
}

/// Server instance representation
pub struct Server {
	pub id: Uuid,
	pub config: RwLock<ServerConfig>,
	pub process: RwLock<ServerProcessState>,
	pub console_lines: RwLock<VecDeque<ConsoleLine>>,
	pub next_line_num: AtomicU64,
}

/// Server process state
/// TODO: Add more states (starting, stopping, etc.) to give more granular info
pub enum ServerProcessState {
	Stopped,
	Running(Arc<ServerRuntime>),
}

impl ServerProcessState {
	pub fn info(&self) -> ServerStateInfo {
		match self {
			ServerProcessState::Stopped => ServerStateInfo::Stopped,
			ServerProcessState::Running(_) => ServerStateInfo::Running,
		}
	}
}

/// Information about the current state of a server
#[derive(Clone, Serialize, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum ServerStateInfo {
	Stopped,
	Running,
}

/// Information about a server instance for listing purposes
#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ServerInfo {
	pub id: Uuid,
	pub config: ServerConfig,
	pub state: ServerStateInfo,
}

impl Server {
	pub async fn info(&self) -> ServerInfo {
		let config = self.config.read().await;
		let state = self.process.read().await.info();

		ServerInfo {
			id: self.id,
			config: config.clone(),
			state,
		}
	}
}
