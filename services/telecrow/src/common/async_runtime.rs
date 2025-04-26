use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct AsyncRuntime {
	handle: Handle,
}

impl AsyncRuntime {
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
pub fn new() -> Arc<AsyncRuntime> {
	Arc::new(AsyncRuntime::new())
}
