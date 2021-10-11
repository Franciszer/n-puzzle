// use crate::map::Map;
use crate::state::State;

pub struct Node<'a> {
	pub parent: Option<&'a Node<'a>>,
	// heuristic score
	pub score: u16,
	pub state: State,
}
