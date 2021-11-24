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
	move_count: u16, // if you move your pawn more than 2^16 - 1 than that is on you not me
}

impl Pawn {
	pub fn new(color: Color) -> Self {
		Pawn { 
			color,
			move_count: 0,
		}
	}
}

impl Piece for Pawn {
	fn move_piece(&mut self, to: Location, from: Location, board: &Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		if self.move_count == 0 && (to.row as i32 - from.row as i32).abs() > 2 {
			return Err("Cannot move pawn more than two spaces on first move");
		} else if self.move_count > 0 && (to.row as i32 - from.row as i32).abs() > 1 {
			return Err("Cannot move pawn more than one space vertically");
		} else if to.row - from.row == 0 && to.col - from.col == 0 {
			return Err("Please don't try and break my code");
		} else if (to.col as i32 - from.col as i32).abs() > 1 {
			return Err("Cannot move pawns diagonally more than one square");
		} else if (to.col as i32 - from.col as i32).abs() == 1 && board[to.row][from.row].is_none() {
			println!("Hi");
			return Err("Can only move diagonally to capture");
		} else if board[to.row][from.row].is_some() {
			return Err("Can only capture diagonally with pawns");
		}
		Ok(false)
	}

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) -> u32 {
		1 
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