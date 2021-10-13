use crate::state::State;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Node {
	pub parent: Option<usize>,
	pub state: Rc<State>,
	pub moves: u16,
}

impl Node {
	pub fn collect_parents(&self, nodes: &Vec<Self>) -> Vec<Rc<State>> {
		let mut states = Vec::new();
		let mut current_node: &Self = self;
		loop {
			states.push(current_node.state.clone());
			current_node = match current_node.parent {
				Some(next_node) => &nodes[next_node],
				None => {
					states.reverse();
					return states;
				}
			}
		}
	}
}

#[cfg(feature = "use_move")]
pub struct ScoreAndIndex {
	pub index: usize,
	pub score: u16,
	pub moves: u16,
}

#[cfg(not(feature = "use_move"))]
pub struct ScoreAndIndex {
	pub index: usize,
	pub score: u16,
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
	#[cfg(feature = "use_move")]
	fn cmp(&self, other: &Self) -> Ordering {
		let mut self_score: u16 = self.moves;
		let mut other_score: u16 = other.moves;
		// self_score *= 2;
		// other_score *= 2;
		self_score += self.score;
		other_score += other.score;
		other_score.cmp(&self_score)
	}
	#[cfg(not(feature = "use_move"))]
	fn cmp(&self, other: &Self) -> Ordering {
		self.score.cmp(&other.score).reverse()
	}
}
