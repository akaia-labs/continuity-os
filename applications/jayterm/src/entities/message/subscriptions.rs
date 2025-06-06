use crowdcomm_sdk::singularity::stdb::{
	DbConnection, EventContext, Message, MessageTableAccess, ReducerEventContext, send_message,
};
use spacetimedb_sdk::{Event, Status, Table};

use crate::common::clients::singularity_client;

pub fn subscribe(ctx: &DbConnection) {
	ctx.db.message().on_insert(on_insert);
	ctx.reducers.on_send_message(on_send);
}

fn on_insert(ctx: &EventContext, message: &Message) {
	if let Event::Reducer(_) = ctx.event {
		singularity_client::print_message(ctx, message)
	}
}

fn on_send(ctx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
