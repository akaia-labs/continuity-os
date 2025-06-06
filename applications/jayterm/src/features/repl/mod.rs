use std::{
	io::{self, BufRead},
	str::FromStr,
};

use corvutils::StringExtensions;
use crowdcomm_sdk::singularity::stdb::{DbConnection, send_message};

use crate::entities::command::{AccountCommand, on_account_command};

/// Starts REPL loop to handle commands and messages.
pub fn start(ctx: &DbConnection) {
	let stdin = io::stdin();
	let handle = stdin.lock();

	for line in handle.lines() {
		let line = match line {
			| Ok(line) => line,

			| Err(e) => {
				let response = format!("Error reading from stdin:\n{e}").padded();

				println!("{response}");
				continue;
			},
		};

		// Detect command marker
		if line.starts_with("/") {
			let parts: Vec<&str> = line[1 ..].splitn(2, " ").collect();

			if parts.is_empty() {
				let response = format!("Invalid command format").padded();

				println!("{response}");
				continue;
			}

			// Parse the command
			if let Ok(command) = AccountCommand::from_str(parts[0]) {
				let args = if parts.len() > 1 {
					parts[1].split_whitespace().map(String::from).collect()
				} else {
					Vec::new()
				};

				match on_account_command(ctx, &command, args) {
					| Ok(_) => (),

					| Err(err) => {
						let response = format!("Command error:\n{err}").padded();

						println!("{response}")
					},
				}
			} else {
				let response = format!("Unknown command:\n{}", parts[0]).padded();

				println!("{response}");
			}
		} else {
			// Not a command, send as a message
			if let Err(err) = ctx.reducers.send_message(line) {
				let response = format!("Error sending message:\n{err}").padded();

				println!("{response}");
			}
		}
	}
}
