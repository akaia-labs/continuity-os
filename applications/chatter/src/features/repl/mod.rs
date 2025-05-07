use crowcomm::crowd_core::{DbConnection, send_message, set_account_callsign};

/// Starts REPL loop to handle commands and messages.
pub fn start(ctx: &DbConnection) {
	for line in std::io::stdin().lines() {
		let Ok(line) = line else {
			panic!("Failed to read from stdin.");
		};

		if let Some(callsign) = line.strip_prefix("/callsign ") {
			ctx.reducers
				.set_account_callsign(callsign.to_string())
				.unwrap();
		} else {
			ctx.reducers.send_message(line).unwrap();
		}
	}
}
