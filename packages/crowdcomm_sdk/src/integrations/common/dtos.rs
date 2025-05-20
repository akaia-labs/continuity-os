use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
	AccountLinkRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResolutionPayload<TCallbackCommand> {
	pub kind:     ActionKind,
	pub callback: TCallbackCommand,
}

impl<TCallbackCommand> ActionResolutionPayload<TCallbackCommand>
where TCallbackCommand: Serialize + for<'de> Deserialize<'de>
{
	/// Deserializes from a JSON string into
	/// `ActionResolutionPayload<TCallbackCommand>`.
	pub fn try_from_str(input: &str) -> Result<Self, String> {
		serde_json::from_str(input).map_err(|e| {
			format!(
				"Failed to deserialize ActionResolutionPayload<{}>: {}",
				std::any::type_name::<TCallbackCommand>(),
				e
			)
		})
	}

	/// Serializes `ActionResolutionPayload<TCallbackCommand>`
	/// into a JSON string.
	pub fn try_to_string(&self) -> Result<String, String> {
		serde_json::to_string(self).map_err(|e| {
			format!(
				"Failed to serialize ActionResolutionPayload<{}>: {}",
				std::any::type_name::<TCallbackCommand>(),
				e
			)
		})
	}
}
