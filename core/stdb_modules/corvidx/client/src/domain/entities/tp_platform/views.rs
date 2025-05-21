use std::fmt::Display;

use crate::common::stdb::TpPlatformTag;

impl Display for TpPlatformTag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.into_supported())
	}
}
