use crate::pieces::*;

pub enum Pieces {
	Pawn(Pawn), 
	Bishop(Bishop),
	Knight(Knight),
	Rook(Rook),
	Queen(Queen),
	King(King),
}