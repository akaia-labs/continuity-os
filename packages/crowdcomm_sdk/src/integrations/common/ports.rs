use corvidx_client::{
	common::stdb::{AccountProfileMetadata, EventContext, ForeignAccountReference},
	domain::entities::foreign_platform::SupportedForeignPlatformTag,
};
pub use teloxide_core::types::Message as TelegramMessage;

pub use crate::corvidx::stdb::Message as CorvidxMessage;

pub trait ForeignAccountImport {
	/// Puts third-party account data into locally recognized format
	fn into_account_reference(&self) -> ForeignAccountReference;
}

pub trait ProfileImport {
	/// Puts third-party profile into locally recognized format
	fn into_profile_metadata(&self) -> AccountProfileMetadata;
}

pub trait MessageConverter<OutboundDto> {
	fn into_corvidx_message(self) -> CorvidxMessage;
	fn from_corvidx_message(
		ctx: &EventContext, msg: &CorvidxMessage, target_platform_tag: SupportedForeignPlatformTag,
	) -> OutboundDto;
}
