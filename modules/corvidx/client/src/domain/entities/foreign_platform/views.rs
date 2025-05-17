use std::fmt::Display;

use crate::common::stdb::ForeignPlatformTag;

impl Display for ForeignPlatformTag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.into_supported())
	}
}
