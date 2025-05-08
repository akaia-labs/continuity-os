use crowdcomm::corvidx;
use spacetimedb_sdk::{Event, Status};

use crate::common::clients::corvidx_client;

pub fn on_message_inserted(ctx: &corvidx::EventContext, message: &corvidx::Message) {
	if let Event::Reducer(_) = ctx.event {
		corvidx_client::print_message(ctx, message)
	}
}

pub fn on_message_sent(ctx: &corvidx::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
