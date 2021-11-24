#![allow(
	dead_code,
	unused_variables,
)]

use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

pub struct King {
	color: Color,
	move_count: u16,
}

impl King {
	pub fn new(color: Color) -> Self {
		King { 
			color,
			move_count: 0,
		}
	}
}

impl Piece for King {
	fn move_piece(&mut self, to: Location, from: Location, board: &Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		Ok(false)
	}

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) -> u32 {
		9001 
	}

	fn piece_to_string(&self) -> String {
		match &self.color {
			Color::White => Colour::White.paint("♔").to_string(),
			Color::Brown => Colour::RGB(165, 42, 42).paint("♔").to_string(),
		}
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