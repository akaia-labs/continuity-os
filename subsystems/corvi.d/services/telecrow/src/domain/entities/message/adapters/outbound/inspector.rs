use crowdcomm_sdk::singularity::stdb::{
	ChannelId, DbConnection, ReducerEventContext, send_message,
};
use spacetimedb_sdk::Status;

pub fn subscribe_to_singularity(ctx: &DbConnection) {
	ctx.reducers.on_send_message(on_send_message);
}

/// Prints a warning if the reducer failed.
fn on_send_message(ctx: &ReducerEventContext, _channel_id: &ChannelId, text: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to send message {:?}: {}", text, err);
	}
}
