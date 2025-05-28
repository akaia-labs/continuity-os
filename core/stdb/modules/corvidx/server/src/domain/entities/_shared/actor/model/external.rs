use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use spacetimedb::SpacetimeType;
use strum_macros::{Display, EnumString};

/// "{String}@{ExternalActorOrigin}"
pub type ExternalActorId = String;

#[derive(SpacetimeType, Clone)]
pub struct ExternalActorReference {
	pub id:     String,
	pub origin: ExternalActorOrigin,
}

#[derive(SpacetimeType, Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalActorOrigin {
	Telegram,
	Unknown,
}

impl ExternalActorReference {
	pub const DELIMITER: char = '@';
}

impl Display for ExternalActorReference {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(
			formatter,
			"{}{}{}",
			self.id,
			Self::DELIMITER,
			self.origin // uses Display from strum
		)
	}
}

pub type ExternalActorReferenceParseErr = &'static str;

impl FromStr for ExternalActorReference {
	type Err = ExternalActorReferenceParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let platform_name_str = parts.next().ok_or("missing platform name")?;
		let id = parts.next().ok_or("missing id")?;

		let origin = platform_name_str
			.parse::<ExternalActorOrigin>()
			.map_err(|_| "invalid or unsupported platform specifier")?;

		Ok(ExternalActorReference {
			id: id.to_owned(),
			origin,
		})
	}
}
