use crate::map::Map;
use crate::state::Point;
use crate::state::State;

struct Evaluator {
	solved_table: Vec<Point>,
	size: u16,
}

impl<'a> Evaluator {
	pub fn build_evaluator(solved_map: &'a Map) -> Evaluator {
		let mut solved_table: Vec::<Point> = vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

		for (i, item) in solved_map.board.iter().enumerate() {
			let p = Point::from_1d(i as u16, solved_map.size);
			// let idx = item as usize;
			solved_table[*item as usize] = p;
		}

		Evaluator {
			solved_table,
			size: solved_map.size,
		}
	}

	pub fn evaluate(&self, state: &State) -> u16 {
		let mut sum: u16 = 0;
		for (i, item) in state.board.iter().enumerate() {
			sum += Point::manhatthan_dist(&Point::from_1d(i as u16, self.size), &self.solved_table[*item as usize]);
		}
		sum
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::map::*;

	#[test]
	fn evaluator_build() {
		let solved = gen_solved_map(5);
		let e = Evaluator::build_evaluator(&solved);
		println!("{:?}", solved);
		println!("len:{} {:?}", e.solved_table.len(), e.solved_table);
	}

	#[test]
	fn evaluate_solved() {
		for i in 3..16 {
			let solved = gen_solved_map(i);
			let e = Evaluator::build_evaluator(&solved);
			let s = State::build_state(solved.board, solved.size);
			assert_eq!(e.evaluate(&s), 0);
		}
	}
}