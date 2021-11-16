use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

#[derive(Clone)]
pub struct Pawn {
	color: Color,
}

impl Pawn {
	pub fn new(c: Color) -> Self {
		Pawn { color: c }
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
	use super::*;

	#[test]
	fn test_display() {
		let pawn = Pawn { color: Color::White};	
		let pawn2 = Pawn { color: Color::Brown};	
		print!("{} {}\n", pawn, pawn2);
	}
}