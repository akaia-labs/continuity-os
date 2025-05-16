use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use crate::common::stdb::{ForeignAccountReference, ForeignPlatformTag};

// TODO: figure out how to reduce the reimplementation overhead

impl ForeignAccountReference {
	pub const DELIMITER: char = '@';
}

impl Display for ForeignAccountReference {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(
			formatter,
			"{}{}{}",
			self.id,
			Self::DELIMITER,
			// ! Temporarily hardcoded
			"telegram".to_string()
		)
	}
}

impl FromStr for ForeignAccountReference {
	type Err = &'static str;

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
