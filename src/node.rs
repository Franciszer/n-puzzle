use crate::state::State;

pub struct Node<'a> {
	pub parent: &'a Node<'a>,
	// heuristic score
	pub score: u16,
	// number of moves from initial state
	pub moves: u16,
	pub state: State,
}
