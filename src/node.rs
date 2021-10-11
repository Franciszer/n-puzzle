use std::cmp::Ordering;

pub struct Node {
	pub parent: Option<usize>,
	pub state: usize,
	// cost
	pub moves: u16,
	// heuristic score
	pub score: u16,
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		(other.score + other.moves).cmp(&(self.score + self.moves))
	}
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
	fn eq(&self, other: &Self) -> bool {
		self.state == other.state
	}
}

impl PartialOrd<Self> for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}
