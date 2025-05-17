use std::str::FromStr;

use crate::common::stdb::{ForeignAccountReference, ForeignPlatformTag};

pub type ForeignAccountReferenceParseErr = &'static str;

impl FromStr for ForeignAccountReference {
	type Err = ForeignAccountReferenceParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let platform_name_str = parts.next().ok_or("missing platform name")?;
		let id = parts.next().ok_or("missing id")?;

		let platform_tag = platform_name_str
			.parse::<ForeignPlatformTag>()
			.map_err(|_| "invalid or unsupported platform specifier")?;

		Ok(ForeignAccountReference {
			id: id.to_owned(),
			platform_tag,
		})
	}
}
