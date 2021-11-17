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
	fn move_piece(&mut self) {
		 
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