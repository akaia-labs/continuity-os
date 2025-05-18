use std::sync::Arc;

use tokio::runtime::Handle;

#[derive(Clone)]
pub struct AsyncHandler {
	handle: Handle,
}

impl AsyncHandler {
	// Create a wrapper to easily create and share a runtime service
	pub fn new() -> Arc<Self> {
		Arc::new(Self {
			handle: Handle::current(),
		})
	}

	pub fn handle(&self) -> Handle {
		self.handle.clone()
	}
}

// pub fn new_async_handler() -> Arc<AsyncHandler> {
// 	Arc::new(AsyncHandler::new())
// }
