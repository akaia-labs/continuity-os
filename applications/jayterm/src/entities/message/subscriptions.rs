use crowdcomm::corvidx;
use spacetimedb_sdk::{Event, Status};

use crate::common::clients::corvidx_client;

pub fn on_message_inserted(corvidx: &corvidx::EventContext, message: &corvidx::Message) {
	if let Event::Reducer(_) = corvidx.event {
		corvidx_client::print_message(corvidx, message)
	}
}

pub fn on_message_sent(corvidx: &corvidx::ReducerEventContext, text: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
