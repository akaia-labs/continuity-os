use serde::{Deserialize, Serialize};

use crate::integrations::dtos::{ActionDescriptor, ActionResolutionCommand};

impl ActionDescriptor {
	/// Deserializes from a JSON string into `ActionDescriptor`.
	pub fn try_from_str(input: &str) -> Result<Self, String> {
		serde_json::from_str(input)
			.map_err(|e| format!("Failed to deserialize ActionDescriptor: {e}"))
	}
}

impl<TResolutionCommand> ActionResolutionCommand<TResolutionCommand>
where TResolutionCommand: Serialize + for<'de> Deserialize<'de>
{
	/// Serializes `ActionResolutionCommand<TResolutionCommand>`
	/// into a JSON string.
	pub fn try_to_string(&self) -> Result<String, String> {
		serde_json::to_string(self).map_err(|e| {
			format!(
				"Failed to serialize ActionResolutionCommand<{callback_command_type}>: {e}",
				callback_command_type = std::any::type_name::<TResolutionCommand>(),
			)
		})
	}

	/// Deserializes from a JSON string into
	/// `ActionResolutionCommand<TResolutionCommand>`.
	pub fn try_from_str(input: &str) -> Result<Self, String> {
		serde_json::from_str(input).map_err(|e| {
			format!(
				"Failed to deserialize ActionResolutionCommand<{callback_command_type}>: {e}",
				callback_command_type = std::any::type_name::<TResolutionCommand>(),
			)
		})
	}
}
