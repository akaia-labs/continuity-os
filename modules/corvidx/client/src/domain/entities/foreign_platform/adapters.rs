use std::str::FromStr;

use super::SupportedForeignPlatformTag;
use crate::common::stdb::ForeignPlatformTag;

impl ForeignPlatformTag {
	pub fn into_supported(&self) -> SupportedForeignPlatformTag {
		(*self).into()
	}
}

impl From<SupportedForeignPlatformTag> for ForeignPlatformTag {
	fn from(name_reference: SupportedForeignPlatformTag) -> Self {
		match name_reference {
			| SupportedForeignPlatformTag::Telegram => ForeignPlatformTag::Telegram,
			| SupportedForeignPlatformTag::Unknown => ForeignPlatformTag::Unknown,
		}
	}
}

impl Into<SupportedForeignPlatformTag> for ForeignPlatformTag {
	fn into(self) -> SupportedForeignPlatformTag {
		match self {
			| ForeignPlatformTag::Telegram => SupportedForeignPlatformTag::Telegram,
			| ForeignPlatformTag::Unknown => SupportedForeignPlatformTag::Unknown,
		}
	}
}

impl FromStr for ForeignPlatformTag {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(ForeignPlatformTag::from(
			match s.parse::<SupportedForeignPlatformTag>() {
				| Ok(platform_tag) => platform_tag,
				| Err(_) => SupportedForeignPlatformTag::Unknown,
			},
		))
	}
}
