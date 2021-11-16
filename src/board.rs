#![allow(unused_variables)]

use std::{fmt, vec};
use ansi_term::Colour;
use crate::{
	enums::color::Color, 
	pieces::pawn::Pawn
};

type PieceLocations<'a, T> = Vec<Vec<Option<&'a T>>>;

#[derive(Debug, Clone)]
pub struct Board<'a, T> {
	board_colors: Vec<Vec<Color>>,
	piece_locations: PieceLocations<'a, T>
}

// T must implement move and remove
impl<'a, T> Board<'a, T> {
	pub fn new() -> Self {
		// board colors
		let row_one = vec![
			Color::White, 
			Color::Brown, 
			Color::White,
			Color::Brown,
			Color::White,
			Color::Brown,
			Color::White,
			Color::Brown
		];
		let row_two = vec![
			Color::Brown, 
			Color::White,
			Color::Brown,
			Color::White,
			Color::Brown,
			Color::White,
			Color::Brown,
			Color::White 
		];



		let mut board_colors = Vec::new();
		for i in 0..8 {
			match i % 2 {
				0 => board_colors.push(row_one.clone()),
				1 => board_colors.push(row_two.clone()),
				_ => panic!("Math is broken")
			};
		}

		let white_pawn_row = vec![Pawn::new(Color::White); 8];
		let brown = vec![Pawn::new(Color::Brown); 8];

		// let white_back_row = vec![];
		// let brown_back_row = vec![];

		// let mut pieces: PieceLocations<T> = vec![
		// 	vec![],
		// 	vec![],
		// 	vec![None; 8],
		// 	vec![None; 8],
		// 	vec![None; 8],
		// 	vec![None; 8],
		// 	vec![Some(&Pawn::new(Color::White)); 8],
		// 	vec![]
		// ] 
		
		Board {
			board_colors,
			piece_locations: vec![],
		}
	}
}

impl<'a, T> fmt::Display for Board<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut result = String::from("\n");
		let mut ct = 1;
		for row in &self.board_colors {
			result = result.to_owned() + &format!("{}   ", Colour::Blue.paint(ct.to_string()));
			for space in row {
				match space {
					Color::White => { result = result.to_owned() + &format!("{} ", Colour::White.paint("x")) }
					Color::Brown => { result = result.to_owned() + &format!("{} ", Colour::RGB(165, 42, 42).paint("x")) }
				}
			}
			result = result.to_owned() + &format!("\n");
			ct += 1;
		}

		result = result.to_owned() + &format!("\n    {}", Colour::Blue.paint("A B C D E F G H"));

		write!(f, "{}", result)
	}
}


#[cfg(test)]
pub mod test_board {
	use super::*;

	#[test]
	fn test_display() {
		// let board = Board::new();
		// println!("{:?}", board);
	}
}