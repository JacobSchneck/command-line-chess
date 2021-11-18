#![allow(
	dead_code
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

pub struct Rook {
	color: Color,
	location: Location,
}

impl Rook {
	pub fn new(color: Color, row: usize, col: usize) -> Self {
		Rook { 
			color,
			location: Location { row, col }
		}
	}
}

impl Piece for Rook {
	// fn move_piece(&mut self) {
		 
	// }

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) {
		 
	}

	fn piece_to_string(&self) -> String {
		match &self.color {
			Color::White => Colour::White.paint("♖").to_string(),
			Color::Brown => Colour::RGB(165, 42, 42).paint("♖").to_string(),
		}
	}
}


impl fmt::Display for Rook {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.color {
			Color::White => write!(f, "{}", Colour::White.paint("♖")),
			Color::Brown => write!(f, "{}", Colour::RGB(165, 42, 42).paint("♖"))
		}
	}
}

#[cfg(test)]
mod test_rook {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let rook = Rook { color: Color::White};	
		// let rook2 = Rook { color: Color::Brown};	
		// print!("{} {}\n", rook, rook2);
	}
}