use crate::solver::Solver;
use crate::map::{gen_solved_map, Map};

pub struct Executor {
	map: Map,
	solver: Solver,
}

impl Executor {
	pub fn new(map: Map) -> Self {
		let solved_map = gen_solved_map(map.size as usize);
		Executor {
			map,
			solver: Solver::new(solved_map),
		}
	}

	pub fn run(&self) {
		if self.solver.is_solvable(&self.map) {
			// println!("SOLVABLE");
			self.solver.solve(self.map.clone());
		} else {
			print!("Puzzle is not solvable !");
		}
	}
}
