use super::SupportedForeignPlatformTag;
use crate::common::stdb::RemoteDbContext;

pub trait PlatformAssociated<T> {
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedForeignPlatformTag,
	) -> Option<T>;
}
