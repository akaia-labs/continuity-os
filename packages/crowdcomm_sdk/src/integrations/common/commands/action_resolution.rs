use serde::{Deserialize, Serialize};
use strum::Display;

use crate::corvidx::account_linking::AccountLinkRequestId;

#[derive(Display, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "choice", content = "request_id")]
pub enum ActionRequestResolution {
	Accept(AccountLinkRequestId),
	Reject(AccountLinkRequestId),
}

impl ActionRequestResolution {
	pub fn label(&self) -> String {
		match self {
			| Self::Accept(_) => "✅ Accept".into(),
			| Self::Reject(_) => "❎ Reject".into(),
		}
	}
}
