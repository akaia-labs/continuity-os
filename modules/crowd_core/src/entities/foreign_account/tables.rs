use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use spacetimedb::{Identity, SpacetimeType, table};

use crate::entities::{account_profile::AccountProfileId, foreign_platform::ForeignPlatformName};

/// "{String}@{ForeignPlatformName}"
pub type ForeignAccountId = String;

#[table(name = foreign_account, public)]
/// Locally recognized format for third-party accounts
pub struct ForeignAccount {
	#[primary_key]
	/// "{String}@{ForeignPlatformName}"
	pub id:         ForeignAccountId,
	#[index(btree)]
	/// Holds username, handle, or any other identifier
	/// with the similar meaning, if present.
	pub callsign:   Option<String>,
	#[index(btree)]
	pub owner_id:   Option<Identity>,
	#[index(btree)]
	pub profile_id: Option<AccountProfileId>,
}

#[derive(SpacetimeType)]
pub struct ForeignAccountReference {
	pub id:            String,
	pub platform_name: ForeignPlatformName,
}

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
			self.platform_name // uses Display from strum
		)
	}
}

impl FromStr for ForeignAccountReference {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let platform_name_str = parts.next().ok_or("missing platform name")?;
		let id = parts.next().ok_or("missing id")?;

		let platform_name = platform_name_str
			.parse::<ForeignPlatformName>()
			.map_err(|_| "invalid or unsupported platform specifier")?;

		Ok(ForeignAccountReference {
			id: id.to_owned(),
			platform_name,
		})
	}
}
