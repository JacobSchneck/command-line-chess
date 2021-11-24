#![allow(
	dead_code,
	unused_imports,
	unused_variables,
)]

use crate::{enums::color::Color, pieces::location};
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
	fn move_piece(&mut self, to: Location, from: Location, board: &mut Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		let moves = vec![
			vec![0, 1],
			vec![0, -1],
			vec![1, 0],
			vec![-1, 0],
			vec![1, 1],
			vec![-1, 1],
			vec![1, -1],
			vec![-1, -1]
		];
		for move_set in moves {
			let next_row = from.row as i32 + move_set[0];
			let next_col = from.col as i32 + move_set[1];
			if next_row >= board.len() as i32 || next_col >= board.len() as i32 { continue; } // move option is out of bounds
			if next_row < 0 || next_col < 0 { continue; } // move option is out of bounds
			if next_row as usize == to.row && next_col as usize == to.col {
				match &board[next_row as usize][next_col as usize] {
					Some(p) => {
						if self.get_color() == p.get_color() {
							return Err("There is no friendly fire in chess my dude, go again"); // redundant error handle is redundunt
						} else {
							return Ok(true);
						}
					},
					None => return Ok(false),
				}
			}
		}
		Err("King can only move one space at a time")
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