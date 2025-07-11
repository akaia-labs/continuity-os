// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::actor_id_type::ActorId;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct CreateDirectChannelArgs {
	pub a_id: ActorId,
	pub b_id: ActorId,
}

impl From<CreateDirectChannelArgs> for super::Reducer {
	fn from(args: CreateDirectChannelArgs) -> Self {
		Self::CreateDirectChannel {
			a_id: args.a_id,
			b_id: args.b_id,
		}
	}
}

impl __sdk::InModule for CreateDirectChannelArgs {
	type Module = super::RemoteModule;
}

pub struct CreateDirectChannelCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `create_direct_channel`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait create_direct_channel {
	/// Request that the remote module invoke the reducer
	/// `create_direct_channel` to run as soon as possible.
	///
	/// This method returns immediately, and errors only if we are unable to
	/// send the request. The reducer will run asynchronously in the future,
	///  and its status can be observed by listening for
	/// [`Self::on_create_direct_channel`] callbacks.
	fn create_direct_channel(&self, a_id: ActorId, b_id: ActorId) -> __sdk::Result<()>;
	/// Register a callback to run whenever we are notified of an invocation of
	/// the reducer `create_direct_channel`.
	///
	/// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the
	/// [`super::ReducerEventContext`] to determine the reducer's status.
	///
	/// The returned [`CreateDirectChannelCallbackId`] can be passed to
	/// [`Self::remove_on_create_direct_channel`] to cancel the callback.
	fn on_create_direct_channel(
		&self,
		callback: impl FnMut(&super::ReducerEventContext, &ActorId, &ActorId) + Send + 'static,
	) -> CreateDirectChannelCallbackId;
	/// Cancel a callback previously registered by
	/// [`Self::on_create_direct_channel`], causing it not to run in the
	/// future.
	fn remove_on_create_direct_channel(&self, callback: CreateDirectChannelCallbackId);
}

impl create_direct_channel for super::RemoteReducers {
	fn create_direct_channel(&self, a_id: ActorId, b_id: ActorId) -> __sdk::Result<()> {
		self.imp
			.call_reducer("create_direct_channel", CreateDirectChannelArgs {
				a_id,
				b_id,
			})
	}

	fn on_create_direct_channel(
		&self,
		mut callback: impl FnMut(&super::ReducerEventContext, &ActorId, &ActorId) + Send + 'static,
	) -> CreateDirectChannelCallbackId {
		CreateDirectChannelCallbackId(self.imp.on_reducer(
			"create_direct_channel",
			Box::new(move |ctx: &super::ReducerEventContext| {
				let super::ReducerEventContext {
					event:
						__sdk::ReducerEvent {
							reducer: super::Reducer::CreateDirectChannel { a_id, b_id },
							..
						},
					..
				} = ctx
				else {
					unreachable!()
				};
				callback(ctx, a_id, b_id)
			}),
		))
	}

	fn remove_on_create_direct_channel(&self, callback: CreateDirectChannelCallbackId) {
		self.imp
			.remove_on_reducer("create_direct_channel", callback.0)
	}
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer
/// `create_direct_channel`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version
/// bump.
pub trait set_flags_for_create_direct_channel {
	/// Set the call-reducer flags for the reducer `create_direct_channel` to
	/// `flags`.
	///
	/// This type is currently unstable and may be removed without a major
	/// version bump.
	fn create_direct_channel(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_create_direct_channel for super::SetReducerFlags {
	fn create_direct_channel(&self, flags: __ws::CallReducerFlags) {
		self.imp
			.set_call_reducer_flags("create_direct_channel", flags);
	}
}
