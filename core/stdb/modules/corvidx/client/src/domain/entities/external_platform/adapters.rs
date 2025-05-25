use std::str::FromStr;

use super::SupportedExternalPlatformTag;
use crate::common::stdb::ExternalPlatformTag;

impl ExternalPlatformTag {
	pub fn into_supported(&self) -> SupportedExternalPlatformTag {
		(*self).into()
	}
}

impl From<SupportedExternalPlatformTag> for ExternalPlatformTag {
	fn from(name_reference: SupportedExternalPlatformTag) -> Self {
		match name_reference {
			| SupportedExternalPlatformTag::Telegram => ExternalPlatformTag::Telegram,
			| SupportedExternalPlatformTag::Unknown => ExternalPlatformTag::Unknown,
		}
	}
}

impl Into<SupportedExternalPlatformTag> for ExternalPlatformTag {
	fn into(self) -> SupportedExternalPlatformTag {
		match self {
			| ExternalPlatformTag::Telegram => SupportedExternalPlatformTag::Telegram,
			| ExternalPlatformTag::Unknown => SupportedExternalPlatformTag::Unknown,
		}
	}
}

impl FromStr for ExternalPlatformTag {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(ExternalPlatformTag::from(
			match s.parse::<SupportedExternalPlatformTag>() {
				| Ok(platform_tag) => platform_tag,
				| Err(_) => SupportedExternalPlatformTag::Unknown,
			},
		))
	}
}
