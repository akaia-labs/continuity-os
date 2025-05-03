mod entities;
mod features;

use entities::{
	internal_account::*,
	public_profile::{PublicProfile, PublicProfileMetadata, PublicProfileOwnerId, public_profile},
};
use spacetimedb::{ReducerContext, Table, reducer};

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
	// Called when the module is initially published
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.account().id().find(ctx.sender) {
		ctx.db.account().id().update(Account {
			is_online: true,
			last_seen_at: ctx.timestamp,
			..account
		});
	} else {
		let initial_profile = ctx.db.public_profile().insert(PublicProfile {
			id:       0,
			owner_id: PublicProfileOwnerId::InternalAccountId(ctx.sender),
			metadata: PublicProfileMetadata::default(),
		});

		let account_profile = ctx.db.public_profile().id().update(PublicProfile {
			//*  Ensuring the profile name is unique at least by default
			metadata: PublicProfileMetadata::default_with_name(format!(
				"{}-{}",
				initial_profile.metadata.name.short_name, initial_profile.id
			)),

			..initial_profile
		});

		ctx.db.account().insert(Account {
			id:           ctx.sender,
			callsign:     format!("0x{}", ctx.sender.to_hex().to_string()),
			role:         AccountRole::Interactor,
			is_online:    true,
			created_at:   ctx.timestamp,
			updated_at:   ctx.timestamp,
			last_seen_at: ctx.timestamp,
			profile_id:   account_profile.id,
		});
	}
}

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
