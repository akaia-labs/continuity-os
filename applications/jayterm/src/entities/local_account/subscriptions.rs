use crowdcomm::corvidx;
use spacetimedb_sdk::Status;

/// If the account is online, prints a notification.
pub fn on_account_inserted(_ctx: &corvidx::EventContext, account: &corvidx::LocalAccount) {
	if account.is_online {
		println!("\nAccount {} connected.\n", account.callsign);
	}
}

/// Prints a notification about callsign and status changes.
pub fn on_account_updated(
	_ctx: &corvidx::EventContext, old: &corvidx::LocalAccount, new: &corvidx::LocalAccount,
) {
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
pub fn on_callsign_set(ctx: &corvidx::ReducerEventContext, callsign: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change callsign to {:?}: {}", callsign, err);
	}
}

// TODO: Error handling for `link_foreign_account` and `unlink_foreign_account`
