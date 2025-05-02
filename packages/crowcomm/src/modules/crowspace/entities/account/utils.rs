use crate::crowspace::{self, PublicProfileTableAccess};

pub fn get_display_name(
	ctx: &impl crowspace::RemoteDbContext, account: &crowspace::Account,
) -> String {
	ctx.db()
		.public_profile()
		.id()
		.find(&account.profile_id)
		.map(|p| {
			if p.metadata.name
				== (crowspace::PublicProfileName {
					short_name:     "Anonymous".to_string(),
					name_extension: None,
				}) {
				account.callsign.clone()
			} else {
				p.metadata.name.to_string()
			}
		})
		.unwrap_or_else(|| account.callsign.clone())
}
