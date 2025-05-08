use std::{fmt::Display, str::FromStr};

use strum_macros::{Display, EnumString};

use crate::corvidx::ForeignPlatformName;

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum SupportedForeignPlatformName {
	Telegram,
	Unknown,
}

impl From<SupportedForeignPlatformName> for ForeignPlatformName {
	fn from(name_reference: SupportedForeignPlatformName) -> Self {
		match name_reference {
			| SupportedForeignPlatformName::Telegram => ForeignPlatformName::Telegram,
			| SupportedForeignPlatformName::Unknown => ForeignPlatformName::Unknown,
		}
	}
}

impl Into<SupportedForeignPlatformName> for ForeignPlatformName {
	fn into(self) -> SupportedForeignPlatformName {
		match self {
			| ForeignPlatformName::Telegram => SupportedForeignPlatformName::Telegram,
			| ForeignPlatformName::Unknown => SupportedForeignPlatformName::Unknown,
		}
	}
}

impl FromStr for ForeignPlatformName {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(ForeignPlatformName::from(
			match s.parse::<SupportedForeignPlatformName>() {
				| Ok(platform_name) => platform_name,
				| Err(_) => SupportedForeignPlatformName::Unknown,
			},
		))
	}
}

impl Display for ForeignPlatformName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", Into::<SupportedForeignPlatformName>::into(*self))
	}
}
