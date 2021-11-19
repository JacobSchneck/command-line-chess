#![allow(
	unused_must_use,
	unused_variables
)]

use std::io::{Write};
use crate::{enums::pieces::Pieces, pieces::location::Location};
use regex::{Regex};

pub fn get_line() -> String {
	let mut input_buffer = String::new();
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut input_buffer).expect("Error says what");
	input_buffer.trim().to_string() // return input
}

pub fn convert_chess_notation_to_indicies(chess_notation: &str) -> Result<Location, &str> {
	let re = Regex::new(r"[a-hA-H][1-8]").unwrap();
	if !re.is_match(chess_notation) { return Err("Invalid chess notation"); } 
	
	let col = chess_notation.chars().nth(0).unwrap().to_ascii_lowercase() as usize - 97;
	let row: usize = chess_notation.chars().nth(1).unwrap() as usize - 49;

	Ok(Location {row, col})
}

pub fn convert_indicies_to_chess_notation(loc: Location) -> Result<String, &'static str> {
	match loc.row {
		0..=7 => (), // wonder if this is bad?
		_ => return Err("Invalid indicies"),
	}; 
	match loc.col {
		0..=7 => (),
		_ => return Err("Invalid indicies"),
	}; 

	let row = loc.row + 1; 
	let col = (loc.row as u8 + 64) as char;
	let result = format!("{}{}", col, row);
	return Ok(result);
}

pub fn str_to_pieces(ch: char) -> Option<Pieces> {
	match ch {
		'p' => Some(Pieces::Pawn),
		'b' => Some(Pieces::Bishop),
		'n' => Some(Pieces::Knight),
		'r' => Some(Pieces::Rook),
		'q' => Some(Pieces::Queen),
		'k' => Some(Pieces::King),
		_ => None,
	}
}

#[cfg(test)]
mod test_utils {
	use super::*;

	#[test]
	fn test_convert_chess_notation_to_indicies() {
		assert_eq!(convert_chess_notation_to_indicies("A2").unwrap(), Location {row: 1, col: 0});
		assert_eq!(convert_chess_notation_to_indicies("I7"), Err("Invalid chess notation"));
	}

	#[test]
	fn test_convert_indicies_to_chess_notation() {
		assert_eq!(convert_indicies_to_chess_notation(Location {row: 1, col: 0}), Ok("A2".to_string()));
		assert_eq!(convert_indicies_to_chess_notation(Location {row: 9, col: 0}), Err("Invalid indicies"));
	}
}