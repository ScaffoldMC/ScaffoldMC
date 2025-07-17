pub mod auth;
pub mod binary;
pub mod server;

trait Service {
	async fn shutdown(&mut self) -> Result<(), String> {
		Ok(())
	}
}
