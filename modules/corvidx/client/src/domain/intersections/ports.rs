use crate::{
	common::stdb::RemoteDbContext, domain::entities::foreign_platform::SupportedForeignPlatformTag,
};

pub trait PlatformAssociation<T> {
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedForeignPlatformTag,
	) -> Option<T>;
}
