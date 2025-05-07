use std::{
	io::{self, BufRead},
	str::FromStr,
};

use crowcomm::crowd_core::{DbConnection, send_message};

use crate::entities::command::{AccountCommand, on_account_command};

/// Starts REPL loop to handle commands and messages.
pub fn start(ctx: &DbConnection) {
	let stdin = io::stdin();
	let handle = stdin.lock();

	for line in handle.lines() {
		let line = match line {
			| Ok(line) => line,
			| Err(e) => {
				eprintln!("Error reading from stdin: {}", e);
				continue;
			},
		};

		// Detect command marker
		if line.starts_with("/") {
			let parts: Vec<&str> = line[1 ..].splitn(2, " ").collect();

			if parts.is_empty() {
				println!("Invalid command format");
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
					| Ok(_) => println!("Command executed successfully"),
					| Err(e) => println!("Command error: {}", e),
				}
			} else {
				println!("Unknown command: {}", parts[0]);
			}
		} else {
			// Not a command, send as a message
			if let Err(e) = ctx.reducers.send_message(line) {
				println!("Error sending message: {}", e);
			}
		}
	}
}
