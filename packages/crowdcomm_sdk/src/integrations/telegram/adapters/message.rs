use capitalize::Capitalize;
use corvidx_client::{
	common::{
		ports::{ProfileResolution, RecordResolution},
		presentation::{DisplayName, Displayable},
		stdb::{AccountLinkRequest, EventContext, MessageAuthorId, TpAccountReference},
	},
	domain::{
		entities::{message::MessageType, tp_platform::SupportedTpPlatformTag},
		intersections::PlatformAssociation,
	},
};
use teloxide_core::types::{ChatId, MessageId, ThreadId};

use crate::integrations::{CorvidxMessage, telegram::OutboundTelegramMessage};

impl OutboundTelegramMessage {
	pub fn from_native(ctx: &EventContext, msg: &CorvidxMessage) -> Self {
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

		let author_name = author_profile
			.map(|profile| profile.display_name())
			.unwrap_or(format!("unknown-{}", msg.sender));

		OutboundTelegramMessage {
			// TODO: `chat_id` and `thread_id` must be taken from MessageChannel
			chat_id:   ChatId(-1001544271932),
			thread_id: Some(ThreadId(MessageId(3315))),

			// TODO: must be taken from Message
			reply_to_message_id: None,

			text: format!(
				"{}\n\n{}",
				format!(
					"{} <strong>{author_name}</strong>",
					MessageType::symbol_by_account_role(author_role)
				),
				msg.text
			),
		}
	}

	pub fn from_account_link_request(
		ctx: &EventContext, alr: &AccountLinkRequest,
	) -> Result<Self, String> {
		let issuer_account = alr
			.issuer
			.resolve(ctx)
			.ok_or("Unable to resolve issuer account.")?;

		let requester_account = alr
			.requester_account_id
			.resolve(ctx)
			.ok_or("Unable to resolve requester account.")?;

		let TpAccountReference {
			id: raw_user_id,
			platform_tag,
		} = alr.subject_account_id
			.parse()
			.map_err(|_| "Unable to parse subject account reference.")?;

		//* Double checking the platform tag
		//* In case of the forwarder letting it through unverified
		if platform_tag.into_supported() != SupportedTpPlatformTag::Telegram {
			return Err(format!(
				"Platform tag {platform_tag} does not match Telegram."
			));
		}

		let subject_user_id: ChatId = raw_user_id
			.parse()
			.map(|user_id: i64| ChatId(user_id))
			.map_err(|_| "Unable to parse subject user id.")?;

		let issuer_name = issuer_account.display_name(ctx);
		let requester_name = requester_account.display_name(ctx);

		Ok(OutboundTelegramMessage {
			chat_id:             subject_user_id,
			thread_id:           None,
			reply_to_message_id: None,

			text: format!(
				"{}\n\n{}",
				format!(
					"{} <strong>{issuer_name}</strong>",
					MessageType::symbol_by_account_role(Some(issuer_account.role)),
				),
				format!(
					r#"
						{requester_name} has requested to link this {platform_name} account.
						If you are the not {requester_name}, please reject this request.
					"#,
					platform_name = SupportedTpPlatformTag::Telegram.to_string().capitalize()
				),
			),
		})
	}
}
