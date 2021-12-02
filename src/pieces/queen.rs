#![allow(
	dead_code,
	unused_variables,
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

#[derive(Clone)]
pub struct Queen {
	color: Color,
	move_count: u32,
}

impl Queen {
	pub fn new(color: Color) -> Self {
		Queen { 
			color,
			move_count: 0,
		}
	}
}

impl Piece for Queen {
	fn move_piece(&mut self, to: Location, from: Location, board: &mut Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		Ok(false)
	}

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) -> u32 {
		11
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
