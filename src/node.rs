use std::cmp::Ordering;
use crate::state::State;
use std::rc::Rc;
use nom::lib::std::hash::{Hash, Hasher};

// #[derive(Hash)]
pub struct Node {
	pub parent: Option<Rc<Node>>,
	pub state: State,
	// cost
	pub moves: u16,
	// heuristic score
	pub score: u16,
}

impl Hash for Node {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.state.hash(state);
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		self.score.cmp(&other.score).reverse()
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
