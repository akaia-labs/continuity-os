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

/// Prints a notification about callsign and status changes.
fn on_update(_corvidx: &EventContext, old: &LocalAccount, new: &LocalAccount) {
	if old.callsign != new.callsign {
		println!(
			"Account {} changed callsign from {} to {}.",
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

fn on_link_foreign_account(
	corvidx: &ReducerEventContext, reference: &ForeignAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	// TODO: Error handling
}

fn on_unlink_foreign_account(
	corvidx: &ReducerEventContext, reference: &ForeignAccountReference, callsign: &Option<String>,
	metadata: &Option<AccountProfileMetadata>,
) {
	// TODO: Error handling
}
