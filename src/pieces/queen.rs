#![allow(
	dead_code
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

pub struct Queen {
	color: Color,
	location: Location,
}

impl Queen {
	pub fn new(color: Color, row: usize, col: usize) -> Self {
		Queen { 
			color,
			location: Location { row, col }
		}
	}
}

impl Piece for Queen {
	fn move_piece(&mut self) {
		 
	}

	fn remove_piece(&mut self) {
		 
	}

	fn piece_to_string(&self) -> String {
		match &self.color {
			Color::White => Colour::White.paint("♕").to_string(),
			Color::Brown => Colour::RGB(165, 42, 42).paint("♕").to_string(),
		}
	}
}

impl fmt::Display for Queen {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.color {
			Color::White => write!(f, "{}", Colour::White.paint("♕")),
			Color::Brown => write!(f, "{}", Colour::RGB(165, 42, 42).paint("♕"))
		}
	}
}

#[cfg(test)]
mod test_queen {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let queen = Queen { color: Color::White};	
		// let queen2 = Queen { color: Color::Brown};	
		// print!("{} {}\n", queen, queen2);
	}
}
