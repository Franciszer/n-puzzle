use crate::map::Map;
use crate::state::Point;

struct Evaluator<'a> {
	solved_map: &'a Map,
	solved_table: Vec<Point>,
}

impl<'a> Evaluator<'a> {
	pub fn build_evaluator(solved_map: &'a Map) -> Evaluator<'a> {
		let mut solved_table: Vec::<Point> = vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

		for (i, item) in solved_map.board.iter().enumerate() {
			let p = Point::from_1d(i as u16, solved_map.size);
			// let idx = item as usize;
			solved_table[*item as usize] = p;
		}

		Evaluator {
			solved_map,
			solved_table,
		}
	}
	// pub fn evaluate(state: &State) {
	//
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::map::*;

	#[test]
	fn eval_basic() {
		let solved = gen_solved_map(5);
		let e = Evaluator::build_evaluator(&solved);
		println!("{:?}", solved);
		println!("{:?}", e.solved_table);
	}
}