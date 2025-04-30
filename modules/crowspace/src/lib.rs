mod entities;

use entities::account::*;
use spacetimedb::{ReducerContext, Table, reducer};

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
	// Called when the module is initially published
}

/// Called when a client connects to the SpacetimeDB
#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.account().identity().find(ctx.sender) {
		// If this is a returning account, i.e. we already have a `Account` with this `Identity`,
		// set `online: true`, but leave `callsign` and `identity` unchanged.
		ctx.db.account().identity().update(Account {
			is_online: true,
			last_seen_at: ctx.timestamp,
			..account
		});
	} else {
		// If this is a new account, create a `Account` row for the `Identity`,
		// which is online, but hasn't set a callsign.
		ctx.db.account().insert(Account {
			callsign: Some(format!("0x{}", ctx.sender.to_hex().to_string())),
			identity: ctx.sender,
			is_online: true,
			updated_at: ctx.timestamp,
			last_seen_at: ctx.timestamp,
		});
	}
}

/// Called when a client disconnects from SpacetimeDB
#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.account().identity().find(ctx.sender) {
		ctx.db.account().identity().update(Account {
			is_online: false,
			last_seen_at: ctx.timestamp,
			..account
		});
	} else {
		// This branch should be unreachable,
		// as it doesn't make sense for a client to disconnect without connecting first.
		log::warn!(
			"Disconnect event for unknown account with identity {:?}",
			ctx.sender
		);
	}
}
