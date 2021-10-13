use crate::map::Map;
use crate::node::{Node, Priority};
use crate::state::Point;
use crate::state::State;
use ahash::AHashSet;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::time::{Duration, Instant};
use crate::heuristic::Heuristic;

pub struct Solver<H: Heuristic> {
	solved_map: Map,
	heuristic: H,
	// size: u16,
}

pub struct Solution<T> {
	pub states: Vec<T>,
	pub size: u16,
	// Number of states opened
	pub time: usize,
	// Number of states stored
	pub memory: usize,
}

impl From<Solution<Rc<State>>> for Solution<State> {
	fn from(to_unwrap: Solution<Rc<State>>) -> Self {
		Self {
			states: to_unwrap
				.states
				.into_iter()
				.map(|e| Rc::<State>::try_unwrap(e).unwrap())
				.collect(),
			size: to_unwrap.size,
			time: to_unwrap.time,
			memory: to_unwrap.memory,
		}
	}
}

impl<H: Heuristic> Solver<H> {
	pub fn new(solved_map: &Map) -> Self {
		let mut solved_table: Vec<Point> =
			vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

		for (i, item) in solved_map.board.iter().enumerate() {
			let p = Point::from_1d(i as u16, solved_map.size);
			solved_table[*item as usize] = p;
		}

		Self {
			solved_map: solved_map.clone(),
			// size: solved_map.size,
			heuristic: H::new(solved_map, solved_map.size),
		}
	}

	/// Computes the score of state using the manhatthan distance
	/// Lower is better
	// pub fn compute_score(&self, state: &State) -> u16 {
	// 	let mut score: u16 = 0;
	// 	for (i, item) in state.board.iter().enumerate() {
	// 		let point = Point::from_1d(i as u16, self.size);
	// 		let dist = Point::manhatthan_dist(&point, &self.solved_table[*item as usize]);
	// 		score += dist;
	// 	}
	// 	score
	// }

	fn get_inv_count(board: &Vec<u16>) -> u16 {
		let mut count: u16 = 0;

		for i in 0..(board.len() - 1) {
			for j in i + 1..board.len() {
				let left = board[i];
				let right = board[j];

				if right != 0 && right > left {
					count += 1;
				}
			}
		}

		count
	}

	#[allow(unreachable_code, unused_variables)]
	pub fn is_solvable(&self, map: &Map) -> bool {
		let inv_count = Self::get_inv_count(&map.board);
		let solved_inv_count = Self::get_inv_count(&self.solved_map.board);

		if inv_count.is_even() == solved_inv_count.is_even() {
			return true
		}

		false
	}

	pub fn solve<P: Priority + Ord>(&self, map: Map) -> Solution<Rc<State>> {
		let size = map.size;
		let root = Rc::new(State::from(map));

		let mut best_score = self.heuristic.compute_score(&root);
		let root_node = Node {
			parent: None,
			state: root.clone(),
			moves: 0,
		};

		let mut nodes = Vec::new();
		let mut states_set: AHashSet<Rc<State>> = AHashSet::new();
		let mut queue: BinaryHeap<P> = BinaryHeap::new();

		nodes.push(root_node);
		queue.push(P::new(0, best_score, 0));
		states_set.insert(root);

		let mut last_print = Instant::now();

		let mut i: usize = 0;
		loop {
			let node_index = queue.pop().unwrap().get_index();
			let state = nodes[node_index].state.clone();
			let moves = nodes[node_index].moves;

			if Instant::now().duration_since(last_print) > Duration::from_secs(1) {
				last_print = Instant::now();
				print!(
					"Distinct: {:9}, Iteration: {:9}, Score: {:3}, Moves: {}\n",
					states_set.len(),
					i,
					best_score,
					moves
				);
				let index = queue.peek().unwrap().get_index();
				println!(
					"{}",
					Map {
						size,
						board: nodes[index].state.board.clone()
					}
				);
			}

			for child in state.gen_children(size) {
				if let Some(state) = child {
					let state = Rc::new(state);
					i += 1;
					if states_set.insert(state.clone()) {
						let score = self.heuristic.compute_score(&state);
						let new_node = Node {
							parent: Some(node_index),
							state,
							moves: moves + 1,
						};
						if score == 0 {
							return Solution {
								states: new_node.collect_parents(&nodes),
								size,
								time: i,
								memory: states_set.len(),
							};
						}
						queue.push(P::new(nodes.len(), score, new_node.moves));
						if score < best_score {
							best_score = score;
						}
						nodes.push(new_node);
					}
				}
			}
		}
	}
}

trait Oddness {
	fn is_odd(&self) -> bool;
	fn is_even(&self) -> bool;
}

impl Oddness for std::primitive::u16 {
	#[inline]
	fn is_odd(&self) -> bool {
		self & 1 == 1
	}

	#[inline]
	fn is_even(&self) -> bool {
		self & 1 == 0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::map::*;

	#[test]
	fn evaluator_build() {
		let solved = gen_solved_map(5);
		let e = Solver::new(&solved);
		println!("{:?}", solved);
		println!("len:{} {:?}", e.solved_table.len(), e.solved_table);
	}

	#[test]
	fn evaluate_unsolved() {
		let mut solved = gen_solved_map(5);
		let e = Solver::new(&solved);

		println!("{:?}", solved);
		solved.board.swap(0, 1);
		let s = State::from(solved);
		assert_eq!(e.compute_score(&s), 2);

		let mut solved = gen_solved_map(5);
		let e = Solver::new(&solved);

		println!("{:?}", solved);
		solved.board.swap(0, 1);
		solved.board.swap(5, 22);
		solved.board.swap(3, 21);
		let s = State::from(solved);
		assert_eq!(e.compute_score(&s), 24);
	}

	#[test]
	fn evaluate_solved() {
		for i in 3..16 {
			let solved = gen_solved_map(i);
			let e = Solver::new(&solved);
			let s = State::from(solved);
			assert_eq!(e.compute_score(&s), 0);
		}
	}
}
