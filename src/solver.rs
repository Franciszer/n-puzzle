// use std::collections::{BinaryHeap, HashMap};
use crate::map::Map;
// use crate::node::Node;
use crate::state::Point;
use crate::state::State;

pub struct Solver {
	solved_map: Map,
	// target positions indexed by value
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
			size,
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

	pub fn is_solvable(&self, map: &Map) -> bool {
		let inv_count = Solver::get_inv_count(&map.board);
		let solved_inv_count = Solver::get_inv_count(&self.solved_map.board);

		// one has to be even and the other one has to be odd
		if inv_count.is_even() != solved_inv_count.is_even() {
			return false
		}
		return true;
	}

	pub fn solve(&self, _map: Map) {
		// let root = State::from(map);
		// TODO: Needs Ord
		// let mut queue: BinaryHeap<Node> = BinaryHeap::new();
		// queue.push(Node {
		// 	parent: None,
		// 	score: self.compute_score(&root),
		// 	state: root,
		// })
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
