use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use spacetimedb::{Identity, SpacetimeType, table};

use crate::entities::{external_platform::ExternalPlatformName, public_profile::PublicProfileId};

/// "{String}@{ExternalPlatformName}"
pub type ExternalAccountId = String;

#[table(name = external_account, public)]
pub struct ExternalAccount {
	#[primary_key]
	/// "{String}@{ExternalPlatformName}"
	pub id:         ExternalAccountId,
	#[index(btree)]
	pub owner_id:   Option<Identity>,
	#[index(btree)]
	pub profile_id: Option<PublicProfileId>,
}

#[derive(SpacetimeType)]
pub struct ExternalAccountReference {
	pub id:            String,
	pub platform_name: ExternalPlatformName,
}

impl ExternalAccountReference {
	pub const DELIMITER: char = '@';
}

impl Display for ExternalAccountReference {
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

impl FromStr for ExternalAccountReference {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let platform_name_str = parts.next().ok_or("missing platform name")?;
		let id = parts.next().ok_or("missing id")?;

		let platform_name = platform_name_str
			.parse::<ExternalPlatformName>()
			.map_err(|_| "invalid platform")?;

		Ok(ExternalAccountReference {
			id: id.to_owned(),
			platform_name,
		})
	}
}
