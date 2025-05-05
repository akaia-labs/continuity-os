use std::{future::Future, pin::Pin, sync::Arc};

use crowcomm::{
	crowd_core::{
		DbConnection, ForeignAccountTableAccess,
		account::ForeignAccountImport,
		import_foreign_account,
		profile::{ProfileImport, ProfileRetrieval},
		update_foreign_account,
	},
	telegram,
};
use teloxide::{Bot, RequestError, respond};

pub fn handle_updates(
	core_ctx: Arc<DbConnection>,
) -> impl Fn(telegram::User, Bot) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send>>
{
	move |tg_user_data: telegram::User, _bot: Bot| {
		let ctx = core_ctx.clone();
		let tg_username = tg_user_data.clone().username;
		let account_reference = tg_user_data.into_account_reference();
		let account_metadata = tg_user_data.into_profile_metadata();

		Box::pin(async move {
			if let Some(account) = ctx
				.db
				.foreign_account()
				.id()
				.find(&account_reference.to_string())
			{
				if account
					.profile(&*ctx)
					.is_some_and(|profile| profile.metadata != account_metadata)
				{
					let _result = ctx.reducers.update_foreign_account(
						account_reference,
						tg_username,
						Some(account_metadata),
					);
				}
			} else {
				let _result = ctx.reducers.import_foreign_account(
					account_reference,
					tg_username,
					Some(account_metadata),
				);
			}

			respond(())
		})
	}
}
