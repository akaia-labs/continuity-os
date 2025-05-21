use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
	AccountLinkRequest,
}

#[derive(Debug, Deserialize)]
/// Used to determine the action kind without deserializing the payload
/// in order to apply the matching deserializer.
pub struct ActionDescriptor {
	pub kind: ActionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionCommand<TPayload> {
	pub kind: ActionKind,

	#[serde(rename = "pl")]
	/// Action resolution command
	pub payload: TPayload,
}
