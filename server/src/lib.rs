mod entities;

use entities::user::*;
use spacetimedb::{ReducerContext, Table, reducer};

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
	// Called when the module is initially published
}

#[reducer(client_connected)]
// Called when a client connects to the SpacetimeDB
pub fn client_connected(ctx: &ReducerContext) {
	if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
		// If this is a returning user, i.e. we already have a `User` with this `Identity`,
		// set `online: true`, but leave `name` and `identity` unchanged.
		ctx.db.user().identity().update(User {
			online: true,
			..user
		});
	} else {
		// If this is a new user, create a `User` row for the `Identity`,
		// which is online, but hasn't set a name.
		ctx.db.user().insert(User {
			name: None,
			identity: ctx.sender,
			online: true,
		});
	}
}

#[reducer(client_disconnected)]
// Called when a client disconnects from SpacetimeDB
pub fn identity_disconnected(ctx: &ReducerContext) {
	if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
		ctx.db.user().identity().update(User {
			online: false,
			..user
		});
	} else {
		// This branch should be unreachable,
		// as it doesn't make sense for a client to disconnect without connecting first.
		log::warn!(
			"Disconnect event for unknown user with identity {:?}",
			ctx.sender
		);
	}
}
