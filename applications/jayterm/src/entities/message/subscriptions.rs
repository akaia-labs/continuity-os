use crowdcomm_sdk::ctx::stdb::{
	DbConnection, EventContext, Message, MessageTableAccess, ReducerEventContext, send_message,
};
use spacetimedb_sdk::{Event, Status, Table};

use crate::common::clients::corvidx_client;

pub fn subscribe(ctx: &DbConnection) {
	ctx.db.message().on_insert(on_insert);
	ctx.reducers.on_send_message(on_send);
}

fn on_insert(ctx: &EventContext, message: &Message) {
	if let Event::Reducer(_) = ctx.event {
		corvidx_client::print_message(ctx, message)
	}
}

fn on_send(ctx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
