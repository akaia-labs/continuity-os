use serde::{Deserialize, Serialize};

use crate::integrations::dtos::{ActionCommand, ActionDescriptor};

impl ActionDescriptor {
	/// Deserializes from a JSON string into `ActionDescriptor`.
	pub fn try_from_str(input: &str) -> Result<Self, String> {
		serde_json::from_str(input)
			.map_err(|e| format!("Failed to deserialize ActionDescriptor: {e}"))
	}
}

impl<TPayload> ActionCommand<TPayload>
where TPayload: Serialize + for<'de> Deserialize<'de>
{
	/// Serializes action command into a JSON string.
	pub fn try_to_string(&self) -> Result<String, String> {
		serde_json::to_string(self).map_err(|e| {
			format!(
				"Failed to serialize ActionCommand<{callback_command_type}>: {e}",
				callback_command_type = std::any::type_name::<TPayload>(),
			)
		})
	}

	/// Deserializes action command from a JSON string.
	pub fn try_from_str(input: &str) -> Result<Self, String> {
		serde_json::from_str(input).map_err(|e| {
			format!(
				"Failed to deserialize ActionCommand<{callback_command_type}>: {e}",
				callback_command_type = std::any::type_name::<TPayload>(),
			)
		})
	}
}
