use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

pub struct Bishop {
	color: Color,
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
	use super::*;

	#[test]
	fn test_display() {
		let bishop = Bishop { color: Color::White};	
		let bishop2 = Bishop { color: Color::Brown};	
		print!("{} {}\n", bishop, bishop2);
	}
}