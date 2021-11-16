use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

pub struct Queen {
	color: Color,
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
	use super::*;

	#[test]
	fn test_display() {
		let queen = Queen { color: Color::White};	
		let queen2 = Queen { color: Color::Brown};	
		print!("{} {}\n", queen, queen2);
	}
}
