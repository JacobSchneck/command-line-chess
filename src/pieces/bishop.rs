#![allow(
	dead_code
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

pub struct Bishop {
	color: Color,
	location: Location,
}

impl Bishop {
	pub fn new(color: Color, row: usize, col: usize) -> Self {
		Bishop { 
			color,
			location: Location { row, col }
		}
	}
}

impl Piece for Bishop {
	fn move_piece(&mut self) {
		 
	}

	fn remove_piece(&mut self) {
		 
	}
}

impl fmt::Display for Bishop {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.color {
			Color::White => write!(f, "{}", Colour::White.paint("♗")),
			Color::Brown => write!(f, "{}", Colour::RGB(165, 42, 42).paint("♗"))
		}
	}
}

#[cfg(test)]
mod test_bishop {
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let bishop = Bishop { color: Color::White};	
		// let bishop2 = Bishop { color: Color::Brown};	
		// print!("{} {}\n", bishop, bishop2);
	}
}