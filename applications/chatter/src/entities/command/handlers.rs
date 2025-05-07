use std::str::FromStr;

use crowcomm::crowd_core::{
	DbConnection, ForeignAccountReference, link_foreign_account, mirror_foreign_profile,
	set_account_callsign, unlink_foreign_account,
};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AccountCommand {
	Callsign,
	LinkAccount,
	UnlinkAccount,
	MirrorForeignProfile,
}

pub fn on_account_command(
	ctx: &DbConnection, command: &AccountCommand, args: Vec<String>,
) -> Result<(), String> {
	match (command, args.len()) {
		| (AccountCommand::Callsign, 1) => ctx
			.reducers
			.set_account_callsign(args[0].clone())
			.map_err(|e| e.to_string()),

		| (AccountCommand::LinkAccount, 1) => {
			let foreign_account_ref = ForeignAccountReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse foreign account id: {e}"))?;

			ctx.reducers
				.link_foreign_account(foreign_account_ref)
				.map_err(|e| e.to_string())
		},

		| (AccountCommand::UnlinkAccount, 1) => {
			let foreign_account_ref = ForeignAccountReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse foreign account id: {e}"))?;

			ctx.reducers
				.unlink_foreign_account(foreign_account_ref)
				.map_err(|e| e.to_string())
		},

		| (AccountCommand::MirrorForeignProfile, 1) => {
			let foreign_account_ref = ForeignAccountReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse foreign account id: {e}"))?;

			ctx.reducers
				.mirror_foreign_profile(foreign_account_ref)
				.map_err(|e| e.to_string())
		},

		| _ => Err(format!("Invalid command.")),
	}
}
