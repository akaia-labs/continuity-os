use capitalize::Capitalize;
use spacetimedb::{ReducerContext, Table, reducer};

use crate::domain::entities::{
	action_request::ExternalAuthenticationRequest,
	channel::DirectChannelReference,
	external_actor::ExternalActorReference,
	shared::{
		keys::{ActorId, ChannelId},
		message::{Message, message},
	},
};

#[reducer]
/// Reports account link resolution outcome.
pub fn report_external_authentication_resolution(
	ctx: &ReducerContext, request: ExternalAuthenticationRequest, is_approved: bool,
) {
	let ExternalAuthenticationRequest {
		requester, subject, ..
	} = request;

	let display_ext_ref = subject
		.parse::<ExternalActorReference>()
		.map_or(subject, |ext_ref| {
			format!(
				"{platform_name} account {fa_id}",
				platform_name = ext_ref.origin.to_string().capitalize(),
				fa_id = ext_ref.id,
			)
		});

	let result = ctx.db.message().try_insert(Message {
		id: 0,

		channel: ChannelId::Direct(
			DirectChannelReference {
				a: ActorId::Internal(ctx.identity()),
				b: ActorId::Internal(requester),
			}
			.to_string(),
		),

		sender:  ctx.identity(),
		sent_at: ctx.timestamp,
		author:  ActorId::Internal(ctx.identity()),

		text: if is_approved {
			format!("{display_ext_ref} has been linked to your account.")
		} else {
			format!("Account link request for {display_ext_ref} has been rejected.")
		},
	});

	if let Err(err) = result {
		log::error!("Failed to send account link resolution message: {err}");
	}
}
