use spacetimedb::ReducerContext;

use super::model::{ExternalAuthenticationRequest, ExternalAuthenticationRequestId, external_authentication_request};
use crate::common::ports::RecordResolution;

impl RecordResolution<ExternalAuthenticationRequest> for ExternalAuthenticationRequestId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<ExternalAuthenticationRequest, String> {
		ctx.db
			.external_authentication_request()
			.id()
			.find(*self)
			.ok_or(format!("Account link request {self} does not exist."))
	}
}
