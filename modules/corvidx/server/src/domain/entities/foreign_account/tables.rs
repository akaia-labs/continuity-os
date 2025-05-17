use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use capitalize::Capitalize;
use spacetimedb::{DbContext, ReducerContext, SpacetimeType, table};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		account_profile::AccountProfileId, foreign_platform::ForeignPlatformTag,
		native_account::NativeAccountId,
	},
};

/// "{String}@{ForeignPlatformTag}"
pub type ForeignAccountId = String;

#[table(name = foreign_account, public)]
/// Locally recognized format for third-party accounts
pub struct ForeignAccount {
	#[primary_key]
	/// "{String}@{ForeignPlatformTag}"
	pub id: ForeignAccountId,

	#[index(btree)]
	/// Holds username, handle, or any other identifier
	/// with the similar meaning, if present.
	pub callsign: Option<String>,

	#[index(btree)]
	pub owner_id: NativeAccountId,

	#[unique]
	#[index(btree)]
	pub profile_id: Option<AccountProfileId>,
}

impl RecordResolution<ForeignAccount> for ForeignAccountId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ForeignAccount, String> {
		let ForeignAccountReference {
			id: external_author_id,
			platform_tag,
		} = self
			.parse()
			.map_err(|e: ForeignAccountReferenceParseErr| e.to_string())?;

		ctx.db().foreign_account().id().find(self).ok_or(format!(
			"{platform_name} account {external_author_id} is not registered in the system.",
			platform_name = platform_tag.to_string().capitalize(),
		))
	}
}

#[derive(SpacetimeType, Clone)]
pub struct ForeignAccountReference {
	pub id:           String,
	pub platform_tag: ForeignPlatformTag,
}

impl RecordResolution<ForeignAccount> for ForeignAccountReference {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ForeignAccount, String> {
		self.to_string().try_resolve(ctx)
	}
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
			self.platform_tag // uses Display from strum
		)
	}
}

type ForeignAccountReferenceParseErr = &'static str;

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
