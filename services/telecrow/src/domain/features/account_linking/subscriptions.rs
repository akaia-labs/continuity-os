use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{AccountLinkRequestTableAccess, DbConnection},
	integrations::{
		CorvidxEventHandler,
		telegram::{OutboundTelegramMessage, TelegramActionRequestForwarder},
	},
	runtime::AsyncHandler,
};
use spacetimedb_sdk::Table;
use teloxide::{
	payloads::SendMessageSetters,
	prelude::Requester,
	types::{InlineKeyboardButton, InlineKeyboardMarkup},
};
use tokio::sync::mpsc;

use crate::BotInstanceType;

/// Sets up account link request forwarding from corvidx to Telegram
/// through a Tokio channel.
pub fn subscribe(
	corvidx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (tx, mut rx) = mpsc::channel::<OutboundTelegramMessage>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(msg) = rx.recv().await {
			let keyboard = InlineKeyboardMarkup::new([[
				InlineKeyboardButton::callback("✅ Accept".to_string(), "accept".to_string()),
				InlineKeyboardButton::callback("❎ Reject".to_string(), "reject".to_string()),
			]]);

			let _ = bridge
				.send_message(msg.chat_id, &msg.text)
				.reply_markup(keyboard)
				.await
				.map_err(|err| eprintln!("{:?}", err));
		}
	});

	let forwarder = TelegramActionRequestForwarder::new(tx, async_handler);

	// Registering the message handler
	corvidx
		.db
		.account_link_request()
		.on_insert(move |ctx, alr| forwarder.handle(ctx, alr));
}
