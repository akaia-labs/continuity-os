use std::{fmt::Display, str::FromStr};

use strum_macros::{Display, EnumString};

use crate::corvidx::ForeignPlatformTag;

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum SupportedForeignPlatformTag {
	Telegram,
	Unknown,
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

impl Display for ForeignPlatformTag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", Into::<SupportedForeignPlatformTag>::into(*self))
	}
}
