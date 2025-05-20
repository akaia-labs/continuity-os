use super::PlatformAssociation;
use crate::{
	common::stdb::{
		NativeAccount, RemoteDbContext, TpAccount, TpAccountReference, TpAccountTableAccess,
	},
	domain::entities::tp_platform::SupportedTpPlatformTag,
};

impl PlatformAssociation<TpAccount> for NativeAccount {
	// TODO: Since one native accounts can have several linked third-party accounts
	// TODO: for the same third-party platform, in the future we'll need
	// TODO: to be able to provide a selector predicate that narrows
	// TODO: the search down to exactly one specific third-party account,
	// TODO: instead of just taking the first found record.
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedTpPlatformTag,
	) -> Option<TpAccount> {
		self.tp_account_ownership
			.iter()
			.filter_map(|account_id| ctx.db().tp_account().id().find(account_id))
			.find(|account| {
				account
					.id
					.parse::<TpAccountReference>()
					.map_or(false, |far| {
						far.platform_tag.into_supported() == platform_tag
					})
			})
	}
}
