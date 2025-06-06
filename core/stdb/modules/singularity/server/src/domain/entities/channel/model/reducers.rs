use spacetimedb::{ReducerContext, reducer};

use super::{
	PrimaryChannel,
	direct::{DirectChannel, direct_channel},
	primary_channel,
	standalone::{StandaloneChannel, standalone_channel},
	subordinate::{SubordinateChannel, delete_subchannel},
};
use crate::{
	common::ports::RecordResolver,
	domain::entities::shared::{keys::ChannelId, message::message},
};

#[reducer]
/// Deletes a channel along with all of its subchannels and messages, if any.
pub fn delete_channel(ctx: &ReducerContext, channel_id: ChannelId) -> Result<(), String> {
	match &channel_id {
		| ChannelId::Direct(id) => {
			let channel: DirectChannel = channel_id.try_resolve(ctx)?;

			for message_id in channel.messages {
				ctx.db.message().id().delete(&message_id);
			}

			ctx.db.direct_channel().id().delete(id);
		},

		| ChannelId::Standalone(id) => {
			let channel: StandaloneChannel = channel_id.try_resolve(ctx)?;

			for message_id in channel.messages {
				ctx.db.message().id().delete(&message_id);
			}

			ctx.db.standalone_channel().id().delete(id);
		},

		| ChannelId::Primary(id) => {
			let channel: PrimaryChannel = channel_id.try_resolve(ctx)?;

			for subchannel_id in channel.subchannels {
				delete_subchannel(ctx, subchannel_id);
			}

			ctx.db.primary_channel().id().delete(id);
		},

		| ChannelId::Subordinate(id) => {
			let subchannel: SubordinateChannel = channel_id.try_resolve(ctx)?;

			let mut superchannel: PrimaryChannel =
				ChannelId::Primary(subchannel.superchannel).try_resolve(ctx)?;

			superchannel
				.subchannels
				.retain(|subchannel_id| subchannel_id != id);

			ctx.db.primary_channel().id().update(superchannel);
			delete_subchannel(ctx, subchannel.id);
		},
	}

	Ok(())
}
