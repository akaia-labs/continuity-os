use spacetimedb::{ReducerContext, Table, reducer};
use uuid::Uuid;

use super::{Channel, channel};

#[reducer]
/// Creates a new channel.
pub fn create_channel(ctx: &ReducerContext) -> Result<(), String> {
	ctx.db.channel().insert(Channel {
		id: Uuid::new_v4().to_string(),
	});

	Ok(())
}
