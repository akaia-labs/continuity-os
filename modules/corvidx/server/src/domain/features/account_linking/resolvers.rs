use spacetimedb::ReducerContext;

use super::tables::{AccountLinkRequest, AccountLinkRequestId, account_link_request};
use crate::common::ports::RecordResolution;

impl RecordResolution<AccountLinkRequest> for AccountLinkRequestId {
	fn try_resolve(&self, ctx: &ReducerContext) -> Result<AccountLinkRequest, String> {
		ctx.db
			.account_link_request()
			.id()
			.find(*self)
			.ok_or(format!("Account link request {self} does not exist."))
	}
}
