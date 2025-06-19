use tokio::process::Child;
use uuid::Uuid;

struct ServerInstance {
	id: Uuid,
	name: String,
	game_version: String,
	process: Option<Child>,
}
