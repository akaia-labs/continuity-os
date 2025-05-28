use spacetimedb::{ReducerContext, Table, reducer};
use uuid::Uuid;

use super::{ChannelKind, PrimaryChannel, StandaloneChannel, primary_channel, standalone_channel};

#[reducer]
/// Creates a new standalone channel.
pub fn create_standalone_channel(ctx: &ReducerContext, kind: ChannelKind) -> Result<(), String> {
	ctx.db.standalone_channel().insert(StandaloneChannel {
		id: Uuid::new_v4().to_string(),
	});

	Ok(())
}

#[reducer]
/// Creates a record for an existing channel
/// bridged from an external source.
pub fn register_channel(ctx: &ReducerContext, kind: ChannelKind) -> Result<(), String> {
	let id = Uuid::new_v4().to_string();

	match kind {
		| ChannelKind::Standalone => {
			ctx.db.standalone_channel().insert(StandaloneChannel { id });
		},

		| ChannelKind::Primary => {
			ctx.db.primary_channel().insert(PrimaryChannel { id });
		},

		| _ => todo!(),
	};

	Ok(())
}
