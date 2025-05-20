use serde::{Deserialize, Serialize};
use strum::Display;

use crate::corvidx::account_linking::AccountLinkRequestId;

#[derive(Display, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "choice", content = "id")]
/// Action request resolution for account link request.
pub enum AlrActionResolution {
	Accept(AccountLinkRequestId),
	Reject(AccountLinkRequestId),
}

impl AlrActionResolution {
	pub fn label(&self) -> String {
		match self {
			| Self::Accept(_) => "✅ Accept".into(),
			| Self::Reject(_) => "❎ Reject".into(),
		}
	}
}
