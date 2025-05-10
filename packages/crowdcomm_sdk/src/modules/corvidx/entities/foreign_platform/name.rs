use std::{fmt::Display, str::FromStr};

use strum_macros::{Display, EnumString};

use crate::corvidx::ForeignPlatformTag;

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum SupportedForeignPlatformName {
	Telegram,
	Unknown,
}

impl From<SupportedForeignPlatformName> for ForeignPlatformTag {
	fn from(name_reference: SupportedForeignPlatformName) -> Self {
		match name_reference {
			| SupportedForeignPlatformName::Telegram => ForeignPlatformTag::Telegram,
			| SupportedForeignPlatformName::Unknown => ForeignPlatformTag::Unknown,
		}
	}
}

impl Into<SupportedForeignPlatformName> for ForeignPlatformTag {
	fn into(self) -> SupportedForeignPlatformName {
		match self {
			| ForeignPlatformTag::Telegram => SupportedForeignPlatformName::Telegram,
			| ForeignPlatformTag::Unknown => SupportedForeignPlatformName::Unknown,
		}
	}
}

impl FromStr for ForeignPlatformTag {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(ForeignPlatformTag::from(
			match s.parse::<SupportedForeignPlatformName>() {
				| Ok(platform_tag) => platform_tag,
				| Err(_) => SupportedForeignPlatformName::Unknown,
			},
		))
	}
}

impl Display for ForeignPlatformTag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", Into::<SupportedForeignPlatformName>::into(*self))
	}
}
