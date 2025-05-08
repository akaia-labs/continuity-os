use crowdcomm::corvidx::{EventContext, LocalAccount, ReducerEventContext};
use spacetimedb_sdk::Status;

/// If the account is online, prints a notification.
pub fn on_account_inserted(_corvidx: &EventContext, account: &LocalAccount) {
	if account.is_online {
		println!("\nAccount {} connected.\n", account.callsign);
	}
}

/// Prints a notification about callsign and status changes.
pub fn on_account_updated(_corvidx: &EventContext, old: &LocalAccount, new: &LocalAccount) {
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
pub fn on_callsign_set(corvidx: &ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

// TODO: Error handling for `link_foreign_account` and `unlink_foreign_account`
