use std::str::FromStr;

use crowdcomm_sdk::corvidx::stdb::{
	DbConnection, ExternalActorReference, initiate_external_authentication,
	mirror_external_profile, set_account_callsign, unlink_external_actor,
};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AccountCommand {
	Callsign,
	LinkAccount,
	UnlinkAccount,
	MirrorExternalProfile,
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
			let ext_actor_ref = ExternalActorReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse third-party account id: {e}"))?;

			corvidx
				.reducers
				.initiate_external_authentication(ext_actor_ref)
				.map_err(|e| e.to_string())
		},

		| (AccountCommand::UnlinkAccount, 1) => {
			let ext_actor_ref = ExternalActorReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse third-party account id: {e}"))?;

			corvidx
				.reducers
				.unlink_external_actor(ext_actor_ref)
				.map_err(|e| e.to_string())
		},

		| (AccountCommand::MirrorExternalProfile, 1) => {
			let ext_actor_ref = ExternalActorReference::from_str(&args[0])
				.map_err(|e| format!("Unable to parse third-party account id: {e}"))?;

			corvidx
				.reducers
				.mirror_external_profile(ext_actor_ref)
				.map_err(|e| e.to_string())
		},

		| _ => Err(format!("Invalid command.")),
	}
}
