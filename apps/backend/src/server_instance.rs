use tokio::process::Child;
use uuid::Uuid;

pub struct ServerInstance {
	id: Uuid,
	name: String,
	game_version: String,
	process: Option<Child>,
}
