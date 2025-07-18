pub mod auth;
pub mod binary;
pub mod java;
pub mod server;

/// Trait for application services.
trait Service {
	/// Called to perform any necessary cleanup before the service is stopped.
	async fn shutdown(&mut self) -> Result<(), String> {
		Ok(())
	}
}
