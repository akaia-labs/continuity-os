use spacetimedb::{ReducerContext, Table, reducer};

use super::{
	account_profile::{AccountProfile, AccountProfileMetadata, account_profile},
	native_account::{NativeAccount, NativeAccountLocalRole, native_account},
};

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
	if let Some(account) = ctx.db.native_account().id().find(ctx.sender) {
		ctx.db.native_account().id().update(NativeAccount {
			is_online: true,
			last_seen_at: ctx.timestamp,
			..account
		});
	} else {
		let initial_profile = ctx.db.account_profile().insert(AccountProfile {
			id:       0,
			metadata: AccountProfileMetadata::default(),
		});

		let account_profile = ctx.db.account_profile().id().update(AccountProfile {
			//*  Ensuring the profile name is unique upon profile creation.
			metadata: AccountProfileMetadata::default_with_name(format!(
				"{}-{}",
				initial_profile.metadata.name.short_name, initial_profile.id
			)),

			..initial_profile
		});

		ctx.db.native_account().insert(NativeAccount {
			id:           ctx.sender,
			callsign:     format!("0x{}", ctx.sender.to_hex().to_string()),
			role:         NativeAccountLocalRole::Interactor,
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
	if let Some(account) = ctx.db.native_account().id().find(ctx.sender) {
		ctx.db.native_account().id().update(NativeAccount {
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
