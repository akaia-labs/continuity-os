use std::sync::Arc;

use crowdcomm_sdk::{
	corvidx::stdb::{DbConnection, MessageTableAccess, ReducerEventContext, send_message},
	integrations::{
		ports::CorvidxEventHandler,
		telegram::{OutboundTelegramMessage, TelegramMessageForwarder},
	},
	runtime::AsyncHandler,
};
use spacetimedb_sdk::{Status, Table};
use teloxide::{payloads::SendMessageSetters, prelude::Requester, sugar::request::RequestReplyExt};
use tokio::sync::mpsc;

use crate::BotInstanceType;

/// Sets up message forwarding from corvidx to Telegram through a Tokio channel.
pub fn forward_to_telegram(
	ctx: &DbConnection, async_handler: Arc<AsyncHandler>, telegram_bot: BotInstanceType,
) {
	let (tx, mut rx) = mpsc::channel::<OutboundTelegramMessage>(100);
	let bridge = telegram_bot.clone();

	// Spawning a background task that processes messages from the channel
	async_handler.handle().spawn(async move {
		while let Some(msg) = rx.recv().await {
			let mut req = bridge.send_message(msg.chat_id, &msg.text);

			if let Some(thread_id) = msg.thread_id {
				req = req.message_thread_id(thread_id)
			};

			if let Some(reply_to_message_id) = msg.reply_to_message_id {
				req = req.reply_to(reply_to_message_id)
			};

			let _ = req.await.map_err(|err| eprintln!("{:?}", err));
		}
	});

	let forwarder = TelegramMessageForwarder::new(tx, async_handler);

	// Registering the message handler
	ctx.db
		.message()
		.on_insert(move |ctx, msg| forwarder.handle(ctx, msg));
}

pub fn inspect(ctx: &DbConnection) {
	ctx.reducers.on_send_message(on_message_sent);
}

/// Prints a warning if the reducer failed.
fn on_message_sent(ctx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
