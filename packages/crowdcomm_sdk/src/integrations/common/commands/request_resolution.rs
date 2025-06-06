use serde::{Deserialize, Serialize};
use strum::Display;

use crate::singularity::external_authentication::ExternalAuthenticationRequestId;

#[derive(Display, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "choice", content = "id")]
/// Action request resolution for account link request.
pub enum ExtAuthReqResolution {
	Accept(ExternalAuthenticationRequestId),
	Reject(ExternalAuthenticationRequestId),
}

impl ExtAuthReqResolution {
	pub fn label(&self) -> String {
		match self {
			| Self::Accept(_) => "✅ Accept".into(),
			| Self::Reject(_) => "❎ Reject".into(),
		}
	}
}
