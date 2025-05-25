use crowdcomm_sdk::corvidx::stdb::{
	Account, AccountTableAccess, DbConnection, EventContext, ReducerEventContext,
	set_account_callsign,
};
use spacetimedb_sdk::{Status, Table, TableWithPrimaryKey};

pub fn subscribe(ctx: &DbConnection) {
	ctx.db.account().on_insert(on_insert);
	ctx.db.account().on_update(on_update);
	ctx.reducers.on_set_account_callsign(on_callsign_set);
}

/// If the account is online, prints a notification.
fn on_insert(_corvidx: &EventContext, account: &Account) {
	if account.is_online {
		println!("\nAccount {} connected.\n", account.callsign);
	}
}

/// Reports account state changes.
fn on_update(_corvidx: &EventContext, old: &Account, new: &Account) {
	if old.callsign != new.callsign {
		println!(
			"\nAccount {} changed callsign from {} to {}.\n",
			old.id, old.callsign, new.callsign,
		);
	}

	if old.is_online && !new.is_online {
		println!("\nAccount {} disconnected.\n", old.callsign);
	}

	if !old.is_online && new.is_online {
		println!("\nAccount {} connected.\n", old.callsign);
	}
}

/// Prints a warning if the reducer failed.
fn on_callsign_set(ctx: &ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}
