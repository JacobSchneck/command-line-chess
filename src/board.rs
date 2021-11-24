#![allow(
	unused_variables,
	unused_imports,
	dead_code,
)]

use std::{fmt, vec};
use ansi_term::Colour;
use crate::{
	enums::{
		color::Color,
		pieces::Pieces,
	}, 
	pieces::{
		pawn::Pawn,
		bishop::Bishop,
		knight::Knight,
		rook::Rook,
		queen::Queen,
		king::King,
		traits::Piece,
		location::Location,
	}, 
	utils::{
		convert_indicies_to_chess_notation,
		piece_check,
	}
};

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
			piece_locations: Board::initialize_board()
		}
	}

	pub fn make_move(&mut self, piece: Pieces, to: Location, from: Location, color: Color) -> Result<(), String> {
		match self.piece_locations[from.row][from. col].take() {
			Some(mut piece_to_move) => {
				match piece_to_move.move_piece(to, from, &self.piece_locations) {
					Ok(b) => (),
					Err(e) => {
						// self.piece_locations[from.row][from.col] = Some(piece_to_move);
						return Err(e.to_string());
					}
				};

				// catching incorrect moves
				if !piece_check(piece, &piece_to_move, piece_to_move.get_color()) {
					self.piece_locations[from.row][from.col] = Some(piece_to_move);
					return Err("Piece intended to move does not match piece at location".to_string());
				} 
				if piece_to_move.get_color() != color {
					self.piece_locations[from.row][from.col] = Some(piece_to_move);
					match color {
						Color::White => return Err(format!("Wrong color it is White's turn")),
						Color::Brown => return Err(format!("Wrong color it is Brown's turn")),
					};
				}
				if let Some(p) = &self.piece_locations[to.row][to.col] {
					if p.get_color() == color {
						self.piece_locations[from.row][from.col] = Some(piece_to_move);
						return Err("There is no friendly fire in chess my dude, go again".to_string());
					}
				}

				self.piece_locations[from.row][from.col] = None;
				self.piece_locations[to.row][to.col] = Some(piece_to_move);
				return Ok(());
			}, // TODO: Move pieces
			// None => return Err(format!("No piece at {}", convert_indicies_to_chess_notation(from).unwrap())), // error case should be handled up stack
			None => {
				let err_msg = format!("No piece at {}", convert_indicies_to_chess_notation(from).unwrap());
				return Err(err_msg);
			}
		}
	}

	fn initialize_board() -> Vec<Vec<Option<Box<dyn Piece>>>> {
		let mut white_pawns: Vec<Option<Box<dyn Piece>>> = Vec::new();
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));
		white_pawns.push(Some(Box::new(Pawn::new(Color::White))));

		let mut brown_pawns: Vec<Option<Box<dyn Piece>>> = Vec::new();
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));
		brown_pawns.push(Some(Box::new(Pawn::new(Color::Brown))));


		let mut white_back_row: Vec<Option<Box<dyn Piece>>> = Vec::new();
		white_back_row.push(Some(Box::new(  Rook::new(Color::White))));
		white_back_row.push(Some(Box::new(Knight::new(Color::White))));
		white_back_row.push(Some(Box::new(Bishop::new(Color::White))));
		white_back_row.push(Some(Box::new( Queen::new(Color::White))));
		white_back_row.push(Some(Box::new(  King::new(Color::White))));
		white_back_row.push(Some(Box::new(Bishop::new(Color::White))));
		white_back_row.push(Some(Box::new(Knight::new(Color::White))));
		white_back_row.push(Some(Box::new(  Rook::new(Color::White))));

		let mut brown_back_row: Vec<Option<Box<dyn Piece>>> = Vec::new();
		brown_back_row.push(Some(Box::new(  Rook::new(Color::Brown))));
		brown_back_row.push(Some(Box::new(Knight::new(Color::Brown))));
		brown_back_row.push(Some(Box::new(Bishop::new(Color::Brown))));
		brown_back_row.push(Some(Box::new( Queen::new(Color::Brown))));
		brown_back_row.push(Some(Box::new(  King::new(Color::Brown))));
		brown_back_row.push(Some(Box::new(Bishop::new(Color::Brown))));
		brown_back_row.push(Some(Box::new(Knight::new(Color::Brown))));
		brown_back_row.push(Some(Box::new(  Rook::new(Color::Brown))));

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
		board.push(white_back_row);
		board.push(white_pawns);
		board.push(empty_row_one);
		board.push(empty_row_two);
		board.push(empty_row_three);
		board.push(empty_row_four);
		board.push(brown_pawns);
		board.push(brown_back_row);
		board // return board
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut result = String::from("\n");
		let mut ct = 1;
		let m = self.piece_locations.len();
		let n = self.piece_locations[0].len();

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