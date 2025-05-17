use corvidx_client::{
	common::{
		ports::{ProfileResolution, RecordResolution},
		presentation::Displayable,
		stdb::{
			AccountProfileMetadata, AccountProfileName, EventContext, ForeignAccountReference,
			ForeignPlatformTag, MessageAuthorId,
		},
	},
	domain::{
		entities::foreign_platform::SupportedForeignPlatformTag, intersections::PlatformAssociation,
	},
};
use teloxide_core::types::{ChatId, User};

use super::OutboundTelegramMessage;
use crate::integrations::{
	CorvidxMessage, ForeignAccountImport, MessageConverter, ProfileImport, TelegramMessage,
};

impl ForeignAccountImport for User {
	fn into_account_reference(&self) -> ForeignAccountReference {
		ForeignAccountReference {
			id:           self.id.to_string(),
			platform_tag: ForeignPlatformTag::Telegram,
		}
	}
}

impl ProfileImport for User {
	fn into_profile_metadata(&self) -> AccountProfileMetadata {
		AccountProfileMetadata {
			name: AccountProfileName {
				short_name:     self.first_name.clone(),
				name_extension: self.last_name.clone(),
			},

			// TODO: Implement bio retrieval
			bio: "".to_string(),
		}
	}
}

impl MessageConverter<OutboundTelegramMessage> for TelegramMessage {
	fn into_corvidx_message(self) -> CorvidxMessage {
		todo!()
	}

	fn from_corvidx_message(
		ctx: &EventContext, msg: &CorvidxMessage, target_platform_tag: SupportedForeignPlatformTag,
	) -> OutboundTelegramMessage {
		let author_profile = match &msg.author_id {
			| MessageAuthorId::ForeignAccountId(account_id) => account_id
				.resolve(ctx)
				.map_or(None, |account| account.profile(ctx)),

			| MessageAuthorId::NativeAccountId(account_id) => account_id
				.resolve(ctx)
				.map(|native_account| {
					native_account
						.platform_association(ctx, target_platform_tag)
						.map_or(native_account.profile(ctx), |foreign_account| {
							foreign_account.profile(ctx)
						})
				})
				.unwrap_or_default(),

			| MessageAuthorId::Unknown => None,
		};

		let author_name = author_profile
			.map(|profile| profile.display_name())
			.unwrap_or(format!("unknown-{}", msg.sender));

		OutboundTelegramMessage {
			// TODO: The chat id must be taken from MessageChannel
			chat_id: ChatId(-1001544271932),

			text: format!(
				"{}\n\n{}",
				format!("💬 <strong>{author_name}</strong>"),
				msg.text
			),
		}
	}
}
