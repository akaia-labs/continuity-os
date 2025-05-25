use std::fmt::Display;

use crate::common::stdb::ExternalPlatformTag;

impl Display for ExternalPlatformTag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.into_supported())
	}
}
