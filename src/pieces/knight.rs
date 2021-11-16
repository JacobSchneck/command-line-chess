use crate::enums::color::Color;
use ansi_term::Colour;
use std::fmt;

pub struct Knight {
	color: Color,
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
	use super::*;

	#[test]
	fn test_display() {
		let kngiht = Knight { color: Color::White};	
		let kngiht2 = Knight { color: Color::Brown};	
		print!("{} {}\n", kngiht, kngiht2);
	}
}