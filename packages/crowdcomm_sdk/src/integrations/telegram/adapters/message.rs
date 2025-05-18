use corvidx_client::{
	common::{
		ports::{ProfileResolution, RecordResolution},
		presentation::Displayable,
		stdb::{EventContext, MessageAuthorId, NativeAccountLocalRole},
	},
	domain::{entities::tp_platform::SupportedTpPlatformTag, intersections::PlatformAssociation},
};
use teloxide_core::types::{ChatId, MessageId, ThreadId};

use crate::integrations::{CorvidxMessage, telegram::OutboundTelegramMessage};

impl OutboundTelegramMessage {
	pub fn from_corvidx(ctx: &EventContext, msg: &CorvidxMessage) -> Self {
		let (author_role, author_profile) = match &msg.author_id {
			| MessageAuthorId::TpAccountId(account_id) => account_id
				.resolve(ctx)
				.map_or((None, None), |account| (None, account.profile(ctx))),

			| MessageAuthorId::NativeAccountId(account_id) => account_id
				.resolve(ctx)
				.map(|native_account| {
					native_account
						.platform_association(ctx, SupportedTpPlatformTag::Telegram)
						.map_or(
							(Some(native_account.role), native_account.profile(ctx)),
							|tp_account| (None, tp_account.profile(ctx)),
						)
				})
				.unwrap_or((None, None)),

			| MessageAuthorId::Unknown => (None, None),
		};

		let message_type_indicator = match author_role {
			| Some(known_role) => match known_role {
				| NativeAccountLocalRole::Admin | NativeAccountLocalRole::Interactor => "💬",
				| NativeAccountLocalRole::Service => "ℹ️",
			},

			| _ => "❓",
		};

		let author_name = author_profile
			.map(|profile| profile.display_name())
			.unwrap_or(format!("unknown-{}", msg.sender));

		OutboundTelegramMessage {
			// TODO: `chat_id` and `thread_id` must be taken from MessageChannel
			chat_id:   ChatId(-1001544271932),
			thread_id: Some(ThreadId(MessageId(3315))),
			// TODO: must be taken from Message
			reply_to:  None,

			text: format!(
				"{}\n\n{}",
				format!("{message_type_indicator} <strong>{author_name}</strong>"),
				msg.text
			),
		}
	}
}
