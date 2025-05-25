use std::str::FromStr;

use crate::common::stdb::{ExternalActorReference, ExternalPlatformTag};

pub type ExternalActorReferenceParseErr = &'static str;

impl FromStr for ExternalActorReference {
	type Err = ExternalActorReferenceParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let platform_name_str = parts.next().ok_or("missing platform name")?;
		let id = parts.next().ok_or("missing id")?;

		let platform_tag = platform_name_str
			.parse::<ExternalPlatformTag>()
			.map_err(|_| "invalid or unsupported platform specifier")?;

		Ok(ExternalActorReference {
			id: id.to_owned(),
			platform_tag,
		})
	}
}
