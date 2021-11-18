use std::io::{Write};

pub fn get_line() -> String {
	let mut input_buffer = String::new();
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut input_buffer).expect("Error says what");
	input_buffer.trim().to_string() // return input
}