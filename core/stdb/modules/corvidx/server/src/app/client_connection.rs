use spacetimedb::{ReducerContext, Table, reducer};

use crate::domain::entities::{
	account::{Account, AccountRole, account},
	actor_profile::{ActorProfile, ActorProfileMetadata, actor_profile},
};

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.account().id().find(ctx.sender) {
		ctx.db.account().id().update(Account {
			is_online: true,
			last_seen_at: ctx.timestamp,
			..account
		});
	} else {
		let initial_profile = ctx.db.actor_profile().insert(ActorProfile {
			id:       0,
			metadata: ActorProfileMetadata::default(),
		});

		let actor_profile = ctx.db.actor_profile().id().update(ActorProfile {
			//*  Ensuring the profile name is unique upon profile creation.
			metadata: ActorProfileMetadata::default_with_name(format!(
				"{}-{}",
				initial_profile.metadata.name.short_name, initial_profile.id
			)),

			..initial_profile
		});

		ctx.db.account().insert(Account {
			id:                ctx.sender,
			callsign:          format!("0x{}", ctx.sender.to_hex().to_string()),
			role:              AccountRole::Interactor,
			is_online:         true,
			created_at:        ctx.timestamp,
			updated_at:        ctx.timestamp,
			last_seen_at:      ctx.timestamp,
			profile:           actor_profile.id,
			exac_associations: vec![],
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
