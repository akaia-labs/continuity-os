use super::foreign_platform::SupportedForeignPlatformTag;
use crate::corvidx::{
	ForeignAccount, ForeignAccountReference, ForeignAccountTableAccess, NativeAccount,
	RemoteDbContext,
};

pub trait PlatformAssociated<T> {
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedForeignPlatformTag,
	) -> Option<T>;
}

impl PlatformAssociated<ForeignAccount> for NativeAccount {
	// TODO: Since one native accounts can have several linked foreign accounts
	// TODO: for the same foreign platform, in the future we'll need
	// TODO: to be able to provide a selector predicate that narrows
	// TODO: the search down to exactly one specific foreign account,
	// TODO: instead of just taking the first found one.
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedForeignPlatformTag,
	) -> Option<ForeignAccount> {
		let res = self
			.foreign_account_ownership
			.iter()
			.filter_map(|account_id| ctx.db().foreign_account().id().find(account_id))
			.find(|account| {
				account
					.id
					.parse::<ForeignAccountReference>()
					.map_or(false, |far| far.platform_tag == platform_tag)
			});
	}
}
