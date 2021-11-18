#![allow(
	dead_code
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

pub struct Knight {
	color: Color,
	location: Location,
}

impl Knight {
	pub fn new(color: Color, row: usize, col: usize) -> Self {
		Knight { 
			color,
			location: Location { row, col }
		}
	}
}

impl Piece for Knight {
	// fn move_piece(&mut self) {
		 
	// }

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) {
		 
	}

	fn piece_to_string(&self) -> String {
		match &self.color {
			Color::White => Colour::White.paint("♘").to_string(),
			Color::Brown => Colour::RGB(165, 42, 42).paint("♘").to_string(),
		}
	}
}

impl fmt::Display for Knight {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.color {
			Color::White => write!(f, "{}", Colour::White.paint("♘")),
			Color::Brown => write!(f, "{}", Colour::RGB(165, 42, 42).paint("♘"))
		}
	}
}

#[cfg(test)]
mod test_knight {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let kngiht = Knight { color: Color::White};	
		// let kngiht2 = Knight { color: Color::Brown};	
		// print!("{} {}\n", kngiht, kngiht2);
	}
}