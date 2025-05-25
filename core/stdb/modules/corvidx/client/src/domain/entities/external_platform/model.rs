use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum SupportedExternalPlatformTag {
	Telegram,
	Unknown,
}
