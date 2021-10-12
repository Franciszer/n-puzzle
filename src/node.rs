use crate::state::State;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Node {
	pub parent: Option<usize>,
	pub state: Rc<State>,
	pub moves: u16,
}

pub struct ScoreAndIndex {
	pub index: usize,
	pub score: u16,
	pub moves: u16,
}

impl Eq for ScoreAndIndex {}

impl PartialEq<Self> for ScoreAndIndex {
	fn eq(&self, other: &Self) -> bool {
		self.score == other.score && self.index == other.index
	}
}

impl PartialOrd<Self> for ScoreAndIndex {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

impl Ord for ScoreAndIndex {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.score * 100 + self.moves)
			.cmp(&(other.score * 100 + self.moves))
			.reverse()
	}
}
