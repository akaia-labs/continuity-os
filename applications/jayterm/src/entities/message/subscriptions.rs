use crowdcomm_sdk::corvidx::stdb::{
	DbConnection, EventContext, Message, MessageTableAccess, ReducerEventContext, send_message,
};
use spacetimedb_sdk::{Event, Status, Table};

use crate::common::clients::corvidx_client;

pub fn subscribe(corvidx: &DbConnection) {
	corvidx.db.message().on_insert(on_insert);
	corvidx.reducers.on_send_message(on_send);
}

fn on_insert(corvidx: &EventContext, message: &Message) {
	if let Event::Reducer(_) = corvidx.event {
		corvidx_client::print_message(corvidx, message)
	}
}

fn on_send(corvidx: &ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
