use crowdcomm_sdk::corvidx::stdb::{
	ActorProfileMetadata, DbConnection, ExternalActorReference, ReducerEventContext,
	register_external_actor, update_external_actor_profile,
};
use spacetimedb_sdk::Status;

fn on_external_actor_import(
	corvidx: &ReducerEventContext, reference: &ExternalActorReference, callsign: &Option<String>,
	metadata: &Option<ActorProfileMetadata>,
) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("\n\nFailed to import account for {:?}: {}", callsign, err);
		println!("\n{:?}", reference);
		println!("{:?}\n\n", metadata);
	}
}

fn on_external_actor_update(
	corvidx: &ReducerEventContext, reference: &ExternalActorReference,
	metadata: &Option<ActorProfileMetadata>,
) {
	if let Status::Failed(err) = &corvidx.event.status {
		eprintln!("\n\nFailed to update account for {reference}: {err}");
		println!("{:?}\n\n", metadata);
	}
}

pub fn subscribe(corvidx: &DbConnection) {
	corvidx
		.reducers
		.on_register_external_actor(on_external_actor_import);

	corvidx
		.reducers
		.on_update_external_actor_profile(on_external_actor_update);
}
