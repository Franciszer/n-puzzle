use crate::map::{gen_solved_map, Map};
use crate::solver::Solver;

use std::io::Write;
use crate::heuristic::{HMannhathan, HHamming};

pub struct Executor {
	map: Map,
	solver: Solver<HMannhathan>,
}

impl Executor {
	pub fn new(map: Map) -> Self {
		let solved_map = gen_solved_map(map.size as usize);
		Executor {
			map,
			solver: Solver::new(&solved_map),
		}
	}

	pub fn run(&self) -> std::io::Result<()> {
		if self.solver.is_solvable(&self.map) {
			let solution = self.solver.solve(self.map.clone());
			for state in solution.states.iter() {
				print!("{:size$}\n", state, size = self.map.size as usize);
			}
			print!(
				"Found solution with {} moves, time complexity: {}, momery complexity: {}\n\n",
				solution.states.len(),
				solution.time,
				solution.memory
			);
			std::io::stdout().flush()
		} else {
			println!("Puzzle is not solvable !");
			Ok(())
		}
	}
}
