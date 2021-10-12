use crate::state::State;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Node {
	pub parent: Option<usize>,
	pub state: Rc<State>,
	// cost
	pub moves: u16,
}

pub struct ScoreAndIndex {
	pub score: u16,
	pub index: usize,
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
		self.score.cmp(&other.score).reverse()
	}
}
