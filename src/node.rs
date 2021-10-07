use crate::state::State;

pub struct Node {
	// pub node: &Node;
	// heuristic score
	pub score: u16,
	// number of moves from initial state
	pub moves: u16,
	pub state: State,
}
