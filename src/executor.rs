use crate::map::{gen_solved_map, Map};
use crate::node::{GreedyPriority, LinearPriority, UniformPriority};
use crate::solver::Solver;
use std::io::Write;

pub struct Executor {
	map: Map,
	solver: Solver,
}

impl Executor {
	pub fn new(map: Map) -> Self {
		let solved_map = gen_solved_map(map.size as usize);
		Executor {
			map,
			solver: Solver::new(&solved_map),
		}
	}

	pub fn run(&self, priority: Priorities) -> std::io::Result<()> {
		if self.solver.is_solvable(&self.map) {
			let solution = match priority {
				Priorities::Linear => self.solver.solve::<LinearPriority>(self.map.clone()),
				Priorities::Uniform => self.solver.solve::<UniformPriority>(self.map.clone()),
				Priorities::Greedy => self.solver.solve::<GreedyPriority>(self.map.clone()),
			};
			for state in solution.states.iter() {
				print!("{:size$}\n", state, size = self.map.size as usize);
			}
			print!(
				"Found solution with {} moves, time complexity: {}, memory complexity: {}\n\n",
				solution.states.len(),
				solution.time,
				solution.memory
			);
			std::io::stdout().flush()
		} else {
			print!("Puzzle is not solvable !");
			Ok(())
		}
	}
}

pub enum Priorities {
	Linear,
	Greedy,
	Uniform,
}
