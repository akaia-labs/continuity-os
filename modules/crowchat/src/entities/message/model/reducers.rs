use super::{tables::*, validators::*};
use spacetimedb::{ReducerContext, Table, reducer};

#[reducer]
/// Clients invoke this reducer to send messages.
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
	let text = validate_message(text)?;

	log::info!("{}", text);

	ctx.db.message().insert(Message {
		id: 0,
		sender: ctx.sender,
		text,
		sent: ctx.timestamp,
	});

	Ok(())
}
