use crowdcomm::corvidx::{
	DbConnection, EventContext, LocalAccount, LocalAccountTableAccess, ReducerEventContext,
	set_account_callsign,
};
use spacetimedb_sdk::{Status, Table, TableWithPrimaryKey};

pub fn subscribe(corvidx: &DbConnection) {
	corvidx.db.local_account().on_insert(on_insert);
	corvidx.db.local_account().on_update(on_update);
	corvidx.reducers.on_set_account_callsign(on_callsign_set);
}

/// If the account is online, prints a notification.
fn on_insert(_corvidx: &EventContext, account: &LocalAccount) {
	if account.is_online {
		println!("\nAccount {} connected.\n", account.callsign);
	}
}

/// Reports account state changes.
fn on_update(_corvidx: &EventContext, old: &LocalAccount, new: &LocalAccount) {
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
fn on_callsign_set(corvidx: &ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}
