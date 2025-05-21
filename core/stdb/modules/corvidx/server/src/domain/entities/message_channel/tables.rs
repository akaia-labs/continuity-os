use spacetimedb::table;

pub type TextChannelId = i128;

#[table(name = text_channel, public)]
pub struct TextChannel {
	#[auto_inc]
	#[primary_key]
	id:   TextChannelId,
	name: String,
}
