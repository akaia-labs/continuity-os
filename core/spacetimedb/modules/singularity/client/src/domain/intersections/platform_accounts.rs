use super::PlatformAssociation;
use crate::{
	common::stdb::{
		Account, ExternalActor, ExternalActorReference, ExternalActorTableAccess, RemoteDbContext,
	},
	domain::entities::external_platform::SupportedExternalActorOrigin,
};

impl PlatformAssociation<ExternalActor> for Account {
	// TODO: Since one internal accounts can have several linked third-party
	// accounts TODO: for the same third-party platform, in the future we'll need
	// TODO: to be able to provide a selector predicate that narrows
	// TODO: the search down to exactly one specific third-party account,
	// TODO: instead of just taking the first found record.
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, origin: SupportedExternalActorOrigin,
	) -> Option<ExternalActor> {
		self.external_actors
			.iter()
			.filter_map(|account_id| ctx.db().external_actor().id().find(account_id))
			.find(|account| {
				account
					.id
					.parse::<ExternalActorReference>()
					.map_or(false, |ext_ref| ext_ref.origin.into_supported() == origin)
			})
	}
}
