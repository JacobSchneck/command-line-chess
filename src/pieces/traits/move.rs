pub trait Move {
	fn move(&self, to: usize, from: usize);
}