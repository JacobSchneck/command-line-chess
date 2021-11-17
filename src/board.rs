#![allow(
	unused_variables,
	unused_imports,
	dead_code,
)]

use std::{fmt, vec};
use ansi_term::Colour;
use crate::{
	enums::color::Color, 
	pieces::pawn::Pawn,
	pieces::bishop::Bishop,
	pieces::knight::Knight,
	pieces::rook::Rook,
	pieces::queen::Queen,
	pieces::king::King,
	pieces::traits::Piece
};

pub fn initialize_board() -> Vec<Vec<Option<Box<dyn Piece>>>> {
	let mut white_pawns: Vec<Option<Box<dyn Piece>>> = Vec::new();
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 0))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 1))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 2))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 3))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 4))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 5))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 6))));
	white_pawns.push(Some(Box::new(Pawn::new(Color::White, 6, 7))));

	let mut brown_pawns: Vec<Option<Box<dyn Piece>>> = Vec::new();
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 0))));
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 1))));
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 2))));
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 3))));
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 4))));
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 5))));
	brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown, 1, 6))));


	let mut white_back_row: Vec<Option<Box<dyn Piece>>> = Vec::new();
	white_back_row.push(Some(Box::new(  Rook::new(Color::White, 7, 0))));
	white_back_row.push(Some(Box::new(Knight::new(Color::White, 7, 1))));
	white_back_row.push(Some(Box::new(Bishop::new(Color::White, 7, 2))));
	white_back_row.push(Some(Box::new( Queen::new(Color::White, 7, 3))));
	white_back_row.push(Some(Box::new(  King::new(Color::White, 7, 4))));
	white_back_row.push(Some(Box::new(Bishop::new(Color::White, 7, 5))));
	white_back_row.push(Some(Box::new(Knight::new(Color::White, 7, 6))));
	white_back_row.push(Some(Box::new(  Rook::new(Color::White, 7, 7))));

	let mut brown_back_row: Vec<Option<Box<dyn Piece>>> = Vec::new();
	brown_back_row.push(Some(Box::new(  Rook::new(Color::Brown, 0, 0))));
	brown_back_row.push(Some(Box::new(Knight::new(Color::Brown, 0, 1))));
	brown_back_row.push(Some(Box::new(Bishop::new(Color::Brown, 0, 2))));
	brown_back_row.push(Some(Box::new( Queen::new(Color::Brown, 0, 3))));
	brown_back_row.push(Some(Box::new(  King::new(Color::Brown, 0, 4))));
	brown_back_row.push(Some(Box::new(Bishop::new(Color::Brown, 0, 5))));
	brown_back_row.push(Some(Box::new(Knight::new(Color::Brown, 0, 6))));
	brown_back_row.push(Some(Box::new(  Rook::new(Color::Brown, 0, 7))));

	let mut empty_row_one: Vec<Option<Box<dyn Piece>>> = Vec::new();
	empty_row_one.push(None);
	empty_row_one.push(None);
	empty_row_one.push(None);
	empty_row_one.push(None);
	empty_row_one.push(None);
	empty_row_one.push(None);
	empty_row_one.push(None);
	empty_row_one.push(None);

	let mut empty_row_two: Vec<Option<Box<dyn Piece>>> = Vec::new();
	empty_row_two.push(None);
	empty_row_two.push(None);
	empty_row_two.push(None);
	empty_row_two.push(None);
	empty_row_two.push(None);
	empty_row_two.push(None);
	empty_row_two.push(None);
	empty_row_two.push(None);

	let mut empty_row_three: Vec<Option<Box<dyn Piece>>> = Vec::new();
	empty_row_three.push(None);
	empty_row_three.push(None);
	empty_row_three.push(None);
	empty_row_three.push(None);
	empty_row_three.push(None);
	empty_row_three.push(None);
	empty_row_three.push(None);
	empty_row_three.push(None);

	let mut empty_row_four: Vec<Option<Box<dyn Piece>>> = Vec::new();
	empty_row_four.push(None);
	empty_row_four.push(None);
	empty_row_four.push(None);
	empty_row_four.push(None);
	empty_row_four.push(None);
	empty_row_four.push(None);
	empty_row_four.push(None);
	empty_row_four.push(None);

	let mut board: Vec<Vec<Option<Box<dyn Piece>>>> = Vec::new();
	board.push(brown_back_row);
	board.push(brown_pawns);
	board.push(empty_row_one);
	board.push(empty_row_two);
	board.push(empty_row_three);
	board.push(empty_row_four);
	board.push(white_pawns);
	board.push(white_back_row);

	board
}

#[derive()]
pub struct Board {
	board_colors: Vec<Vec<Color>>,
	piece_locations: Vec<Vec<Option<Box<dyn Piece>>>>,
}

// T must implement move and remove
impl Board {

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

		Board {
			board_colors,
			piece_locations: initialize_board()
		}
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut result = String::from("\n");
		let mut ct = 1;
		let m = self.piece_locations.len();
		let n = self.piece_locations[0].len();
		println!("{} {}", m, n);

		for i in 0..m {
			result = result.to_owned() + &format!("{}   ", Colour::Blue.paint(ct.to_string()));
			for j in 0..n {
				match &self.piece_locations[i][j] {
					Some(piece) => { result = result.to_owned() + &format!("{} ", piece.piece_to_string())}
					None => {
						match &self.board_colors[i][j] {
							Color::White => { result = result.to_owned() + &format!("{} ", Colour::White.paint("-")) }
							Color::Brown => { result = result.to_owned() + &format!("{} ", Colour::RGB(165, 42, 42).paint("-")) }
						}
					}
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
		let board = Board::new();
		println!("{}", board);
	}
}