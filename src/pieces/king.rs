#![allow(
	dead_code
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

pub struct King {
	color: Color,
	location: Location,
}

impl King {
	pub fn new(color: Color, row: usize, col: usize) -> Self {
		King { 
			color,
			location: Location { row, col }
		}
	}
}

impl Piece for King {
	fn move_piece(&mut self) {
		 
	}

	fn remove_piece(&mut self) {
		 
	}
}

impl fmt::Display for King {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.color {
			Color::White => write!(f, "{}", Colour::White.paint("♔")),
			Color::Brown => write!(f, "{}", Colour::RGB(165, 42, 42).paint("♔"))
		}
	}
}

#[cfg(test)]
mod test_king {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let king = King { color: Color::White};	
		// let king2 = King { color: Color::Brown};	
		// print!("{} {}\n", king, king2);
	}
}