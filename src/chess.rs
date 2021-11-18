use super::enums::color::Color;
use super::board::Board;
use super::utils::get_line;
use super::pieces::traits::Piece;
use regex::Regex;
// use super::pieces::Pieces;

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
		println!("To move: <piece> <row><col> to <row><col>");
		println!("Where <piece> is selected via:");
		println!("Pawn = P");
		println!("Bishop = B");
		println!("Knight = N");
		println!("Rook = R");
		println!("Queen = Q");
		println!("King = K");
		println!("Use the digits for <row> and letters for <col>");
		println!("Example: P E2 to E4");

		println!("To quit type q");
		println!("For help type help");
	}

	pub fn play(&self) {
		Chess::help();
		let re = Regex::new(r"[a-zA-Z] [a-zA-Z]\d [a-zA-Z]\d").unwrap();
		loop {
			println!("{}", &self.board);
			print!("> ");
			let line = get_line();
			println!();
			// match line.as_str() {

			// 	"help" => Chess::help(),
			// 	"q" => break,
			// 	_ => println!("Invalid command, type help for help")
			// };
			if re.is_match(&line) {
				println!("Executing line");
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