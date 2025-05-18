use spacetimedb::{ReducerContext, Table, reducer};

use crate::domain::entities::{
	account_profile::{
		AccountProfile, AccountProfileMetadata, AccountProfileName, account_profile,
	},
	native_account::{NativeAccount, NativeAccountLocalRole, native_account},
};

#[reducer(init)]
/// Called when the module is initially published.
pub fn init(ctx: &ReducerContext) {
	ctx.db.native_account().insert(NativeAccount {
		id:                        ctx.identity(),
		callsign:                  "corvidx".to_string(),
		role:                      NativeAccountLocalRole::Service,
		is_online:                 true,
		last_seen_at:              ctx.timestamp,
		created_at:                ctx.timestamp,
		updated_at:                ctx.timestamp,
		tp_account_ownership: vec![],

		profile_id: ctx
			.db
			.account_profile()
			.insert(AccountProfile {
				id: 0,

				metadata: AccountProfileMetadata {
					name: AccountProfileName {
						short_name:     "Corvi.d".to_string(),
						name_extension: None,
					},

					bio: "🐦‍⬛ <-- sees everything".to_string(),
				},
			})
			.id,
	});
}
