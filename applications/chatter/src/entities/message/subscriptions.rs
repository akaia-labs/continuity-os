use crowcomm::crowd_core;
use spacetimedb_sdk::{Event, Status};

use crate::common::clients::crowd_core_client;

pub fn on_message_inserted(ctx: &crowd_core::EventContext, message: &crowd_core::Message) {
	if let Event::Reducer(_) = ctx.event {
		crowd_core_client::print_message(ctx, message)
	}
}

pub fn on_message_sent(ctx: &crowd_core::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
