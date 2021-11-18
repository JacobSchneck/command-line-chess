use super::location::Location;
use crate::enums::color::Color;

pub trait Piece {
	fn move_piece(&mut self, to: Location, board: Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		Ok(false)
	}

	fn get_color(&self) -> Color;

	fn remove_piece(&mut self) {

	}

	fn piece_to_string(&self) -> String;
}