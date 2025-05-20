use std::str::FromStr;

use crowdcomm_sdk::corvidx::stdb::{
	DbConnection, TpAccountReference, create_account_link_request, mirror_tp_profile,
	set_account_callsign, unlink_tp_account,
};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AccountCommand {
	Callsign,
	LinkAccount,
	UnlinkAccount,
	MirrorTpProfile,
}

pub fn on_account_command(
	corvidx: &DbConnection, command: &AccountCommand, args: Vec<String>,
) -> Result<(), String> {
	match (command, args.len()) {
		| (AccountCommand::Callsign, 1) => corvidx
			.reducers
			.set_account_callsign(args[0].clone())
			.map_err(|e| e.to_string()),

		| (AccountCommand::LinkAccount, 1) => {
			let tp_account_ref = TpAccountReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse third-party account id: {e}"))?;

			corvidx
				.reducers
				.create_account_link_request(tp_account_ref)
				.map_err(|e| e.to_string())
		},

		| (AccountCommand::UnlinkAccount, 1) => {
			let tp_account_ref = TpAccountReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse third-party account id: {e}"))?;

			corvidx
				.reducers
				.unlink_tp_account(tp_account_ref)
				.map_err(|e| e.to_string())
		},

		| (AccountCommand::MirrorTpProfile, 1) => {
			let tp_account_ref = TpAccountReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse third-party account id: {e}"))?;

			corvidx
				.reducers
				.mirror_tp_profile(tp_account_ref)
				.map_err(|e| e.to_string())
		},

		| _ => Err(format!("Invalid command.")),
	}
}
