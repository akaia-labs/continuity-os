use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use capitalize::Capitalize;
use spacetimedb::{DbContext, ReducerContext, SpacetimeType, table};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{
		account_profile::AccountProfileId, tp_platform::TpPlatformTag,
		native_account::NativeAccountId,
	},
};

/// "{String}@{TpPlatformTag}"
pub type TpAccountId = String;

#[table(name = tp_account, public)]
/// Locally recognized format for third-party accounts
pub struct TpAccount {
	#[primary_key]
	/// "{String}@{TpPlatformTag}"
	pub id: TpAccountId,

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

impl RecordResolution<TpAccount> for TpAccountId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<TpAccount, String> {
		let TpAccountReference {
			id: external_author_id,
			platform_tag,
		} = self
			.parse()
			.map_err(|e: TpAccountReferenceParseErr| e.to_string())?;

		ctx.db().tp_account().id().find(self).ok_or(format!(
			"{platform_name} account {external_author_id} is not registered in the system.",
			platform_name = platform_tag.to_string().capitalize(),
		))
	}
}

#[derive(SpacetimeType, Clone)]
pub struct TpAccountReference {
	pub id:           String,
	pub platform_tag: TpPlatformTag,
}

impl RecordResolution<TpAccount> for TpAccountReference {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<TpAccount, String> {
		self.to_string().try_resolve(ctx)
	}
}

impl TpAccountReference {
	pub const DELIMITER: char = '@';
}

impl Display for TpAccountReference {
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

type TpAccountReferenceParseErr = &'static str;

impl FromStr for TpAccountReference {
	type Err = TpAccountReferenceParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.rsplitn(2, Self::DELIMITER);
		let platform_name_str = parts.next().ok_or("missing platform name")?;
		let id = parts.next().ok_or("missing id")?;

		let platform_tag = platform_name_str
			.parse::<TpPlatformTag>()
			.map_err(|_| "invalid or unsupported platform specifier")?;

		Ok(TpAccountReference {
			id: id.to_owned(),
			platform_tag,
		})
	}
}
