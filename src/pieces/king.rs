use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

pub struct King {
	color: Color,
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
	use super::*;

	#[test]
	fn test_display() {
		let king = King { color: Color::White};	
		let king2 = King { color: Color::Brown};	
		print!("{} {}\n", king, king2);
	}
}