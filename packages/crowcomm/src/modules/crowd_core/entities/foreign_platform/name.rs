use std::str::FromStr;

use strum_macros::{Display, EnumString};

use crate::crowd_core::ForeignPlatformName;

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
