use super::{
	PrimaryChannel, direct::DirectChannel, standalone::StandaloneChannel,
	subordinate::SubordinateChannel,
};
use crate::{
	common::ports::{RecordResolver, Resolvable},
	domain::entities::shared::keys::ChannelId,
};

impl Resolvable for ChannelId {
	fn try_is_resolvable(&self, ctx: &spacetimedb::ReducerContext) -> Result<(), String> {
		match self {
			| ChannelId::Direct(_) => {
				let result: Result<DirectChannel, String> = self.try_resolve(ctx);

				result.map(|_| ())
			},

			| ChannelId::Standalone(_) => {
				let result: Result<StandaloneChannel, String> = self.try_resolve(ctx);

				result.map(|_| ())
			},

			| ChannelId::Primary(_) => {
				let result: Result<PrimaryChannel, String> = self.try_resolve(ctx);

				result.map(|_| ())
			},

			| ChannelId::Subordinate(_) => {
				let result: Result<SubordinateChannel, String> = self.try_resolve(ctx);

				result.map(|_| ())
			},
		}
	}
}
