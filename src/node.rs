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

pub trait Priority {
	fn get_index(&self) -> usize;
	fn new(index: usize, score: u16, moves: u16) -> Self;
}

pub struct LinearPriority {
	index: usize,
	score: u16,
	moves: u16,
}

pub struct UniformPriority {
	index: usize,
	moves: u16,
}

pub struct GreedyPriority {
	index: usize,
	score: u16,
}

impl Priority for LinearPriority {
	fn get_index(&self) -> usize {
		self.index
	}

	fn new(index: usize, score: u16, moves: u16) -> Self {
		Self {
			index,
			score,
			moves,
		}
	}
}

impl Priority for UniformPriority {
	fn get_index(&self) -> usize {
		self.index
	}

	fn new(index: usize, _: u16, moves: u16) -> Self {
		Self { index, moves }
	}
}

impl Priority for GreedyPriority {
	fn get_index(&self) -> usize {
		self.index
	}

	fn new(index: usize, score: u16, _: u16) -> Self {
		Self { index, score }
	}
}

impl Eq for LinearPriority {}

impl PartialEq<Self> for LinearPriority {
	fn eq(&self, other: &Self) -> bool {
		self.score == other.score && self.index == other.index
	}
}

impl PartialOrd<Self> for LinearPriority {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

impl Ord for LinearPriority {
	fn cmp(&self, other: &Self) -> Ordering {
		(other.score + other.moves).cmp(&(self.score + self.moves))
	}
}

impl Eq for UniformPriority {}

impl PartialEq<Self> for UniformPriority {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl PartialOrd<Self> for UniformPriority {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

impl Ord for UniformPriority {
	fn cmp(&self, other: &Self) -> Ordering {
		other.moves.cmp(&self.moves)
	}
}

impl Eq for GreedyPriority {}

impl PartialEq<Self> for GreedyPriority {
	fn eq(&self, other: &Self) -> bool {
		self.score == other.score && self.index == other.index
	}
}

impl PartialOrd<Self> for GreedyPriority {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

impl Ord for GreedyPriority {
	fn cmp(&self, other: &Self) -> Ordering {
		other.score.cmp(&self.score)
	}
}
