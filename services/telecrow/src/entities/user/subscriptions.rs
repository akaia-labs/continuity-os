use crowlink::clients::crownest;
use spacetimedb_sdk::Status;

use super::model::*;

/// If the user is online, prints a notification.
pub fn on_user_inserted(_ctx: &crownest::EventContext, user: &crownest::User) {
	if user.online {
		println!("User {} connected.", user_name_or_identity(user));
	}
}

/// Prints a notification about name and status changes.
pub fn on_user_updated(_ctx: &crownest::EventContext, old: &crownest::User, new: &crownest::User) {
	if old.name != new.name {
		println!(
			"User {} renamed to {}.",
			user_name_or_identity(old),
			user_name_or_identity(new)
		);
	}

	if old.online && !new.online {
		println!("User {} disconnected.", user_name_or_identity(new));
	}

	if !old.online && new.online {
		println!("User {} connected.", user_name_or_identity(new));
	}
}

/// Prints a warning if the reducer failed.
pub fn on_name_set(ctx: &crownest::ReducerEventContext, name: &String) {
	if let Status::Failed(err) = &ctx.event.status {
		eprintln!("Failed to change name to {:?}: {}", name, err);
	}
}
