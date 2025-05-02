use spacetimedb::SpacetimeType;
use strum_macros::{Display, EnumString};

#[derive(SpacetimeType, Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalPlatformName {
	Telegram,
	Unknown,
}
