use std::fmt::Display;

use crate::common::stdb::ExternalActorOrigin;

impl Display for ExternalActorOrigin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.into_supported())
	}
}
