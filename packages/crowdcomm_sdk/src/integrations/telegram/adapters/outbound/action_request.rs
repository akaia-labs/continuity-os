use capitalize::Capitalize;
use corvidx_client::{
	common::{
		ports::RecordResolution,
		presentation::DisplayName,
		stdb::{AccountLinkRequest, EventContext, TpAccountReference},
	},
	domain::entities::{message::MessageType, tp_platform::SupportedTpPlatformTag},
};
use corvutils::StringExtensions;
use teloxide_core::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

use super::OutboundTelegramActionRequest;
use crate::integrations::telegram::AccountLinkRequestCallback;

impl OutboundTelegramActionRequest {
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
		let accept_callback = AccountLinkRequestCallback::Accept(alr.id);
		let reject_callback = AccountLinkRequestCallback::Reject(alr.id);

		Ok(OutboundTelegramActionRequest {
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
				)
				.squash_whitespace(),
			),

			reply_markup: InlineKeyboardMarkup::new([[
				InlineKeyboardButton::callback(
					accept_callback.label(),
					accept_callback.try_to_json()?,
				),
				InlineKeyboardButton::callback(
					reject_callback.label(),
					reject_callback.try_to_json()?,
				),
			]]),
		})
	}
}
