use spacetimedb::table;

pub type TextChannelId = u64;

#[table(name = text_channel, public)]
pub struct TextChannel {
	#[auto_inc]
	#[primary_key]
	id: TextChannelId,
	name: String,
}
