use crate::map::Map;
use crate::state::Point;
use crate::state::State;

pub struct Solver {
	solved_table: Vec<Point>,
	size: u16,
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
		let solved = gen_solved_map(5);
		let e = Solver::new(&solved);

		println!("{:?}", solved);
		let mut v = solved.board.clone();
		v.swap(0, 1);
		let s = State::build_state(v, solved.size);
		assert_eq!(e.compute_score(&s), 2);

		let solved = gen_solved_map(5);
		let e = Solver::new(&solved);

		println!("{:?}", solved);
		let mut v = solved.board.clone();
		v.swap(0, 1);
		v.swap(5, 22);
		v.swap(3, 21);
		let s = State::build_state(v, solved.size);
		assert_eq!(e.compute_score(&s), 24);
	}

	#[test]
	fn evaluate_solved() {
		for i in 3..16 {
			let solved = gen_solved_map(i);
			let e = Solver::new(&solved);
			let s = State::build_state(solved.board, solved.size);
			assert_eq!(e.compute_score(&s), 0);
		}
	}
}
