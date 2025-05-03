use std::sync::Arc;

use tokio::runtime::Handle;

#[derive(Clone)]
pub struct AsyncHandler {
	handle: Handle,
}

impl AsyncHandler {
	pub fn new() -> Self {
		Self {
			handle: Handle::current(),
		}
	}

	pub fn handle(&self) -> Handle {
		self.handle.clone()
	}
}

// Create a wrapper to easily create and share a runtime service
pub fn new_async_handler() -> Arc<AsyncHandler> {
	Arc::new(AsyncHandler::new())
}

pub type TelecrowError = Box<dyn std::error::Error + Send + Sync>;
