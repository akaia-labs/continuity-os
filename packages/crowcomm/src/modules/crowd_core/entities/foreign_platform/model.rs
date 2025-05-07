use std::str::FromStr;

use strum_macros::{Display, EnumString};

use crate::crowd_core::ForeignPlatformName;

// TODO: figure out how to reduce the reimplementation overhead

// #[derive(Debug, Clone, PartialEq, Display, EnumString)]
// #[strum(serialize_all = "lowercase")]
// pub enum SupportedForeignPlatformName {
// 	Telegram,
// 	Unknown,
// }

impl FromStr for ForeignPlatformName {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.parse::<ForeignPlatformName>()

		// match s {
		// 	| SupportedForeignPlatformName::Telegram =>
		// Ok(ForeignPlatformName::Telegram),
		// 	| SupportedForeignPlatformName::Unknown =>
		// Ok(ForeignPlatformName::Unknown), 	| _ => Err("invalid platform
		// name"), }
	}
}
