use crate::{
	common::stdb::RemoteDbContext, domain::entities::tp_platform::SupportedTpPlatformTag,
};

pub trait PlatformAssociation<T> {
	fn platform_association(
		&self, ctx: &impl RemoteDbContext, platform_tag: SupportedTpPlatformTag,
	) -> Option<T>;
}
