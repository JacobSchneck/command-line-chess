#![allow(
	dead_code,
	unused_variables,
)]

use crate::{
	enums::color::Color,
	utils::in_bounds,
};

use ansi_term::Colour;
use std::fmt;

use super::{location::Location, traits::Piece};

#[derive(Clone)]
pub struct Knight {
	color: Color,
	move_count: u16,
}

impl Knight {
	pub fn new(color: Color) -> Self {
		Knight { 
			color,
			move_count: 0,
		}
	}
}

impl Piece for Knight {
	fn move_piece(&mut self, to: Location, from: Location, board: &mut Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		let increments: Vec<Vec<i8>> = vec![
			vec![2, 1],
			vec![2, -1],
			vec![1, 2],
			vec![-1, 2],
			vec![-2, 1],
			vec![-2, -1],
			vec![1, -2],
			vec![-1, -2],
		];

		let mut valid_moves = vec![vec![false; 8]; 8];
		for i in 0..increments.len() {
			let mut next_row = from.row as i8 + increments[i][0];
			let mut next_col = from.col as i8 + increments[i][1];
			while in_bounds(next_row, next_col) {
				valid_moves[next_row as usize][next_col as usize] = true;
				next_row += increments[i][0];
				next_col += increments[i][1];
			}
			if in_bounds(next_row, next_col) {
				if let Some(p) = board[next_row as usize][next_col as usize].as_ref() {
					if p.get_color() != self.color {
						valid_moves[next_row as usize][next_col as usize] = true;
					}
				}
			}
		}

		if !valid_moves[to.row][to.col] {
			return Err("Knight can't do that, learn to play chess you dummy!")
		}
		if board[to.row][to.col].is_some() {
			return Ok(true);
		}
		Ok(false)
	}

	fn get_color(&self) -> Color {
		self.color
	}

	fn remove_piece(&mut self) -> u32 {
		3 
	}

	fn piece_to_string(&self) -> String {
		match &self.color {
			Color::White => Colour::White.paint("♘").to_string(),
			Color::Brown => Colour::RGB(165, 42, 42).paint("♘").to_string(),
		}
	}
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
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_display() {
		// let kngiht = Knight { color: Color::White};	
		// let kngiht2 = Knight { color: Color::Brown};	
		// print!("{} {}\n", kngiht, kngiht2);
	}
}