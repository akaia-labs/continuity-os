use super::foreign_platform::SupportedForeignPlatformTag;
use crate::corvidx::{ForeignAccount, ForeignAccountTableAccess, NativeAccount, RemoteDbContext};

pub trait PlatformAssociated<T> {
	fn platform_association(&self, platform_tag: SupportedForeignPlatformTag) -> Option<T>;
}

impl PlatformAssociated<ForeignAccount> for NativeAccount {
	// TODO: Since one native accounts can have several linked foreign accounts
	// TODO: for the same foreign platform, in the future we'll need
	// TODO: to be able to provide a selector closure that narrows
	// TODO: the search down to one specific foreign account
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedForeignPlatformTag,
	) -> ForeignAccount {
		ctx.db().foreign_account().id().find(&self.id).unwrap()
	}
}
