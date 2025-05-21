use std::str::FromStr;

use super::SupportedTpPlatformTag;
use crate::common::stdb::TpPlatformTag;

impl TpPlatformTag {
	pub fn into_supported(&self) -> SupportedTpPlatformTag {
		(*self).into()
	}
}

impl From<SupportedTpPlatformTag> for TpPlatformTag {
	fn from(name_reference: SupportedTpPlatformTag) -> Self {
		match name_reference {
			| SupportedTpPlatformTag::Telegram => TpPlatformTag::Telegram,
			| SupportedTpPlatformTag::Unknown => TpPlatformTag::Unknown,
		}
	}
}

impl Into<SupportedTpPlatformTag> for TpPlatformTag {
	fn into(self) -> SupportedTpPlatformTag {
		match self {
			| TpPlatformTag::Telegram => SupportedTpPlatformTag::Telegram,
			| TpPlatformTag::Unknown => SupportedTpPlatformTag::Unknown,
		}
	}
}

impl FromStr for TpPlatformTag {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(TpPlatformTag::from(
			match s.parse::<SupportedTpPlatformTag>() {
				| Ok(platform_tag) => platform_tag,
				| Err(_) => SupportedTpPlatformTag::Unknown,
			},
		))
	}
}
