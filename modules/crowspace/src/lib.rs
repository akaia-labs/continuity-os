mod entities;
mod features;

use entities::{
	account_profile::{
		AccountProfile, AccountProfileMetadata, AccountProfileOwnerId, account_profile,
	},
	internal_account::*,
};
use spacetimedb::{ReducerContext, Table, reducer};

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
	// Called when the module is initially published
}

/// Called when a client connects to the SpacetimeDB
#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.account().id().find(ctx.sender) {
		ctx.db.account().id().update(Account {
			is_online: true,
			last_seen_at: ctx.timestamp,
			..account
		});
	} else {
		let account_profile = ctx.db.account_profile().insert(AccountProfile {
			id:       0,
			owner_id: AccountProfileOwnerId::InternalAccountId(ctx.sender),
			metadata: AccountProfileMetadata::default(),
		});

		ctx.db.account().insert(Account {
			id:           ctx.sender,
			callsign:     Some(format!("0x{}", ctx.sender.to_hex().to_string())),
			role:         AccountRole::Interactor,
			is_online:    true,
			created_at:   ctx.timestamp,
			updated_at:   ctx.timestamp,
			last_seen_at: ctx.timestamp,
			profile:      account_profile,
		});
	}
}

/// Called when a client disconnects from SpacetimeDB
#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.account().id().find(ctx.sender) {
		ctx.db.account().id().update(Account {
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
