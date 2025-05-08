use crowdcomm::corvidx::{
	DbConnection, EventContext, ForeignAccountReference, LocalAccount, LocalAccountTableAccess,
	ReducerEventContext, link_foreign_account, set_account_callsign, unlink_foreign_account,
};
use spacetimedb_sdk::{Status, Table, TableWithPrimaryKey};

pub fn subscribe(corvidx: &DbConnection) {
	corvidx.db.local_account().on_insert(on_insert);
	corvidx.db.local_account().on_update(on_update);

	corvidx.reducers.on_set_account_callsign(on_callsign_set);

	corvidx
		.reducers
		.on_link_foreign_account(on_link_foreign_account);

	corvidx
		.reducers
		.on_unlink_foreign_account(on_unlink_foreign_account);
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

fn on_link_foreign_account(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	match &corvidx.event.status {
		| Status::Committed => {
			print!("\nForeign account {reference} successfully linked to your account.\n")
		},

		| Status::Failed(err) => {
			eprintln!("\nUnable to link foreign account {reference}: {}\n", err)
		},

		| _ => {},
	}
}

fn on_unlink_foreign_account(corvidx: &ReducerEventContext, reference: &ForeignAccountReference) {
	match &corvidx.event.status {
		| Status::Committed => {
			print!("\nForeign account {reference} successfully unlinked from your account.\n")
		},

		| Status::Failed(err) => {
			eprintln!("\nUnable to unlink foreign account {reference}: {}\n", err)
		},

		| _ => {},
	}
}
