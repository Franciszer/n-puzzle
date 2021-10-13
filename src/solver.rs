use crate::map::Map;
use crate::node::{Node, ScoreAndIndex};
use crate::state::Point;
use crate::state::State;
use ahash::AHashSet;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Solver {
	solved_table: Vec<Point>,
	size: u16,
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

impl Solver {
	pub fn new(solved_map: &Map) -> Solver {
		let mut solved_table: Vec<Point> =
			vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

		for (i, item) in solved_map.board.iter().enumerate() {
			let p = Point::from_1d(i as u16, solved_map.size);
			solved_table[*item as usize] = p;
		}

		Solver {
			solved_table,
			size: solved_map.size,
		}
	}

	/// Computes the score of state using the manhatthan distance
	/// Lower is better
	pub fn compute_score(&self, state: &State) -> u16 {
		let mut score: u16 = 0;
		for (i, item) in state.board.iter().enumerate() {
			let point = Point::from_1d(i as u16, self.size);
			let dist = Point::manhatthan_dist(&point, &self.solved_table[*item as usize]);
			score += dist;
		}
		score
	}

	fn get_inv_count(&self, board: &Vec<u16>) -> u16 {
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
		return true;
		let inv_count = self.get_inv_count(&map.board);

		if map.size.is_odd() {
			inv_count.is_even()
		} else {
			let pos: u16 = map.board.iter().position(|&r| r == 0).unwrap() as u16;
			if pos.is_odd() {
				inv_count.is_even()
			} else {
				inv_count.is_odd()
			}
		}
	}

	pub fn solve(&self, map: Map) -> Solution<Rc<State>> {
		let size = map.size;
		let root = Rc::new(State::from(map));

		let mut best_score = self.compute_score(&root);
		let root_node = Node {
			parent: None,
			state: root.clone(),
			moves: 0,
		};

		let mut nodes = Vec::new();
		let mut states_set: AHashSet<Rc<State>> = AHashSet::new();
		let mut queue: BinaryHeap<ScoreAndIndex> = BinaryHeap::new();

		nodes.push(root_node);
		queue.push(ScoreAndIndex {
			score: best_score,
			index: 0,
			#[cfg(feature = "use_move")]
			moves: 0,
		});
		states_set.insert(root);

		let mut last_print = Instant::now();

		let mut i: usize = 0;
		loop {
			let ScoreAndIndex {
				index: node_index, ..
			} = queue.pop().unwrap();
			let state = nodes[node_index].state.clone();
			let moves = nodes[node_index].moves;

			if Instant::now().duration_since(last_print) > Duration::from_secs(1) {
				last_print = Instant::now();
				print!(
					"Distinct: {:9}, Iteration: {:9}, Score: {:3}\n",
					states_set.len(),
					i,
					best_score
				);
				let index = queue.peek().unwrap().index;
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
						let score = self.compute_score(&state);
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
						queue.push(ScoreAndIndex {
							index: nodes.len(),
							score,
							#[cfg(feature = "use_move")]
							moves: moves + 1,
						});
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
