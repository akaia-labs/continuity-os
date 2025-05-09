use corvutils::StringExtensions;

pub fn print_success(log_message: String) {
	println!("{text}", text = log_message.squash_whitespace().padded())
}

pub fn print_error(log_message: String) {
	eprintln!("{text}", text = log_message.squash_whitespace().padded())
}
