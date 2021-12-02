#![allow(
	unused_variables,
)]

use super::location::Location;
use crate::enums::color::Color;

pub trait Piece: PieceClone {
	fn move_piece(&mut self, to: Location, from: Location, board: &mut Vec<Vec<Option<Box<dyn Piece>>>>) -> Result<bool, &str> {
		Ok(false)
	}

	fn get_color(&self) -> Color;

	fn remove_piece(&mut self) -> u32;

	fn piece_to_string(&self) -> String;
}

pub trait PieceClone {
	fn clone_box(&self) -> Box<dyn Piece>;
}

impl<T> PieceClone for T
where
	T: 'static + Piece + Clone
{
	fn clone_box(&self) -> Box<dyn Piece> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Piece> {
	fn clone(&self) -> Box<dyn Piece> {
		self.clone_box()
	}
}