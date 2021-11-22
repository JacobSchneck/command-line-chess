#![allow(
	unused_variables,
	dead_code
)]

use crate::utils::str_to_pieces;

use super::enums::color::Color;
use super::board::Board;
use super::utils::{ get_line, convert_chess_notation_to_indicies };
use super::pieces::traits::Piece;
use regex::Regex;

pub struct Chess {
	board: Board,
	turn: Color,
	white_pieces_captured: Vec<Box<dyn Piece>>,
	brown_pieces_captured: Vec<Box<dyn Piece>>,
}


impl Chess {
	pub fn new() -> Self {
		Chess { 
			board: Board::new(),
			turn: Color::White,
			white_pieces_captured: Vec::new(),
			brown_pieces_captured: Vec::new(),
		}
	}

	fn help() {
		println!("To move: <piece> <col><row> <col><row>");
		println!("Where <piece> is selected via:");
		println!("Pawn = P");
		println!("Bishop = B");
		println!("Knight = N");
		println!("Rook = R");
		println!("Queen = Q");
		println!("King = K");
		println!("Use the digits for <row> and letters for <col>");
		println!("Example: P E2 E4 | moves pawn on E2 to E4");

		println!("To quit type q");
		println!("For help type help");
	}

	fn execute_command(&mut self, command: &str) {
		// parse command
		let split_command: Vec<String> = command.split(' ').map(String::from).collect();
		// println!("{:?}", split_command);
		let piece = str_to_pieces(split_command[0].chars().nth(0).unwrap()).expect("Invalid Command");
		let from = convert_chess_notation_to_indicies(&split_command[1]).unwrap();
		let to = convert_chess_notation_to_indicies(&split_command[2]).unwrap();
		// println!("{:?} {:?}", to, from);
		match self.board.make_move(piece, to, from, self.turn) {
			Ok(_) => {
				if self.turn == Color::White {
					self.turn = Color::Brown;
				} else {
					self.turn = Color::White;
				}
			},
			Err(e) => eprintln!("{}", e),
		}
	}

	pub fn play(&mut self) {
		Chess::help();
		let re = Regex::new(r"[pbnrqkPBNRQK] [a-hA-H][1-8] [a-hA-H][1-8]").unwrap();
		loop {
			println!("{}", &self.board);
			print!("> ");
			let line = get_line();
			println!();
			if re.is_match(&line) {
				self.execute_command(&line);
			}
			else if line == "help" {
				Chess::help();
			} else if line == "q" {
				break;
			} else {
				println!("Invalid command");
			}
		}
	}
}