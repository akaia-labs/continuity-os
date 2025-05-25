use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use capitalize::Capitalize;
use spacetimedb::{DbContext, ReducerContext, SpacetimeType, table};
use strum_macros::{Display, EnumString};

use crate::{
	common::ports::RecordResolution,
	domain::entities::{account::AccountId, actor_profile::ActorProfileId},
};

/// "{String}@{ExternalActorOrigin}"
pub type ExternalActorId = String;

#[table(name = external_actor, public)]
/// Locally recognized format for third-party accounts
pub struct ExternalActor {
	#[primary_key]
	/// "{String}@{ExternalActorOrigin}"
	pub id: ExternalActorId,

	#[index(btree)]
	/// Holds username, handle, or any other identifier
	/// with the similar meaning, if present.
	pub callsign: Option<String>,

	#[index(btree)]
	pub account: Option<AccountId>,

	#[unique]
	#[index(btree)]
	pub profile: Option<ActorProfileId>,
}

impl RecordResolution<ExternalActor> for ExternalActorId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ExternalActor, String> {
		let ExternalActorReference {
			id: external_author_id,
			origin,
		} = self
			.parse()
			.map_err(|e: ExternalActorReferenceParseErr| e.to_string())?;

		ctx.db().external_actor().id().find(self).ok_or(format!(
			"{platform_name} account {external_author_id} is not registered in the system.",
			platform_name = origin.to_string().capitalize(),
		))
	}
}

#[derive(SpacetimeType, Clone)]
pub struct ExternalActorReference {
	pub id:           String,
	pub origin: ExternalActorOrigin,
}

#[derive(SpacetimeType, Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalActorOrigin {
	Telegram,
	Unknown,
}

impl RecordResolution<ExternalActor> for ExternalActorReference {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ExternalActor, String> {
		self.to_string().try_resolve(ctx)
	}
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

type ExternalActorReferenceParseErr = &'static str;

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
