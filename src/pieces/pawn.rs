#![allow(
	unused_variables
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

#[derive(Clone)]
pub struct Pawn {
	color: Color,
	location: Location,
}

impl Pawn {
	pub fn new(color: Color, row: usize, col: usize) -> Self {
		Pawn { 
			color,
			location: Location { row, col }
		}
	}
}

impl Piece for Pawn {
	fn move_piece(&mut self, to: Location, board: Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		// // Handle Invalid moves
		// if to.row >= board.len() || { // 
		// 	return Err("Invalid Move");
		// if (self.location.row as i32 - to.row as i32).abs() > 2 { // no moving more than 2 spaces at once
		// 	return Err("Invalid Move");
		// } else if (self.location.row as i32 - to.row as i32).abs() > 1 && (self.location.col as i32 - to.col as i32).abs() > 1 { // move diagonally case
		// 	match &board[to.row][to.col] {
		// 		Some(p) => {
		// 			if p.get_color() == self.get_color() {
		// 				return Err("Invalid Move");
		// 			}
		// 		},
		// 		None => return Err("Invalid Move")
		// 	}
		// 	return Ok(true);
		// } else if to.col != self.location.col { // can't move diagonally unless opponent piece is off by one 
		// 	return Err("Invalid Move");
		// } else if ()
		match &to.row {
			0..=7 => println!("HELLO"),
			_ => return Err("Invalid Move"),
		};

		Ok(false)
	}

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) {
		 
	}

	fn piece_to_string(&self) -> String {
		match &self.color {
			Color::White => Colour::White.paint("♙").to_string(),
			Color::Brown => Colour::RGB(165, 42, 42).paint("♙").to_string(),
		}
	}
}

impl fmt::Display for Pawn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.color {
			Color::White => write!(f, "{}", Colour::White.paint("♙")),
			Color::Brown => write!(f, "{}", Colour::RGB(165, 42, 42).paint("♙"))
		}
	}
}


#[cfg(test)]
mod test_pawn {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let pawn = Pawn::new(Color::White, 0, 0);
		// let pawn = Pawn::new(Color::Brown, 1, 1);
		// print!("{} {}\n", pawn, pawn2);
	}
}