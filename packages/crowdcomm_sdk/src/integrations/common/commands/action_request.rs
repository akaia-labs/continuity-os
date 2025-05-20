use serde::{Deserialize, Serialize};
use strum::Display;

use crate::corvidx::account_linking::AccountLinkRequestId;

#[derive(Display, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "resolution", content = "request_id")]
pub enum AccountLinkRequestCallback {
	Accept(AccountLinkRequestId),
	Reject(AccountLinkRequestId),
}

impl AccountLinkRequestCallback {
	pub fn label(&self) -> String {
		match self {
			| Self::Accept(_) => "✅ Accept".into(),
			| Self::Reject(_) => "❎ Reject".into(),
		}
	}

	pub fn try_to_json(&self) -> Result<String, String> {
		serde_json::to_string(self).map_err(|e| {
			format!("Failed to serialize resolution command for account link request {self}: {e}")
		})
	}

	pub fn try_from_str(input: &str) -> Result<Self, String> {
		serde_json::from_str(input).map_err(|e| {
			format!("Failed to deserialize account link request resolution command {input}: {e}")
		})
	}
}
