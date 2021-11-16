use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

pub struct Rook {
	color: Color,
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
	use super::*;

	#[test]
	fn test_display() {
		let rook = Rook { color: Color::White};	
		let rook2 = Rook { color: Color::Brown};	
		print!("{} {}\n", rook, rook2);
	}
}