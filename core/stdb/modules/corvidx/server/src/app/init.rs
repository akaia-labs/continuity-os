use spacetimedb::{ReducerContext, Table, reducer};

use crate::domain::entities::{
	account::{Account, AccountRole, account},
	shared::actor::{ActorName, ActorProfile, ActorProfileMetadata, actor_profile},
};

#[reducer(init)]
/// Called when the module is initially published.
pub fn init(ctx: &ReducerContext) {
	ctx.db.account().insert(Account {
		id:              ctx.identity(),
		callsign:        "corvidx".to_string(),
		role:            AccountRole::Service,
		is_online:       true,
		last_seen_at:    ctx.timestamp,
		created_at:      ctx.timestamp,
		updated_at:      ctx.timestamp,
		external_actors: vec![],

		profile: ctx
			.db
			.actor_profile()
			.insert(ActorProfile {
				id: 0,

				metadata: ActorProfileMetadata {
					name: ActorName {
						short_name:     "Corvi.d".to_string(),
						name_extension: None,
					},

					description: "🐦‍⬛ <-- sees everything".to_string(),
				},
			})
			.id,
	});
}
