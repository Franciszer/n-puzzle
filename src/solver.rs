use crate::map::Map;
use crate::node::Node;
use crate::state::Point;
use crate::state::State;
use ahash::AHasher;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
// use std::time::{Duration, Instant};

pub struct Solver {
	solved_map: Map,
	solved_table: Vec<Point>,
	size: u16,
}

impl Solver {
	pub fn new(solved_map: Map) -> Solver {
		let mut solved_table: Vec<Point> =
			vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

		for (i, item) in solved_map.board.iter().enumerate() {
			let p = Point::from_1d(i as u16, solved_map.size);
			solved_table[*item as usize] = p;
		}

		let size = solved_map.size;
		Solver {
			solved_map,
			solved_table,
			size: size,
		}
	}

	/// Computes the score of state using the manhatthan distance
	/// Lower is better
	fn compute_score(&self, state: &State) -> u16 {
		let mut score: u16 = 0;
		for (i, item) in state.board.iter().enumerate() {
			score += Point::manhatthan_dist(
				&Point::from_1d(i as u16, self.size),
				&self.solved_table[*item as usize],
			);
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

	pub fn is_solvable(&self, map: &Map) -> bool {
		let inv_count = self.get_inv_count(&map.board);
		let solved_inv_count = self.get_inv_count(&self.solved_map.board);

		if inv_count.is_even() == solved_inv_count.is_even() {
			return true;
		}

		false
	}

	pub fn solve(&self, map: Map) {
		let size = map.size;
		let mut transpose_table = HashSet::new();
		let mut priority_queue = BinaryHeap::new();

		let root = State::from(map);
		let root_node = Rc::new(Node {
			parent: None,
			score: self.compute_score(&root),
			state: root,
			moves: 0,
		});

		transpose_table.insert(Rc::clone(&root_node));
		priority_queue.push(Rc::clone(&root_node));

		loop {
			let current = priority_queue.pop().unwrap();
			// println!("{:?}", current.state);

			let children = current.state.gen_children(size);

			for child in children {
				if let Some(child) = child {
					let ptr = Rc::new(Node {
						parent: Some(Rc::clone(&current)),
						score: self.compute_score(&child),
						state: child,
						moves: current.moves + 1,

					});
					let inserted = transpose_table.insert(Rc::clone(&ptr));
					if inserted == true {
						if ptr.score == 0 {
							println!("OK");
							println!("{}", Map {
								size: self.size,
								board: ptr.state.board.clone()
							});
							return
						}
						priority_queue.push(Rc::clone(&ptr));
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

fn hash<T: Hash>(item: &T) -> u64 {
	let mut hasher = AHasher::default();
	item.hash(&mut hasher);
	hasher.finish()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::map::*;

	#[test]
	fn evaluator_build() {
		let solved = gen_solved_map(5);
		let e = Solver::new(solved.clone());
		println!("{:?}", solved);
		println!("len:{} {:?}", e.solved_table.len(), e.solved_table);
	}

	#[test]
	fn evaluate_unsolved() {
		let mut solved = gen_solved_map(5);
		let e = Solver::new(solved.clone());

		println!("{:?}", solved);
		solved.board.swap(0, 1);
		let s = State::from(solved);
		assert_eq!(e.compute_score(&s), 2);

		let mut solved = gen_solved_map(5);
		let e = Solver::new(solved.clone());

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
			let e = Solver::new(solved.clone());
			let s = State::from(solved);
			assert_eq!(e.compute_score(&s), 0);
		}
	}
}
