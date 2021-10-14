use crate::map::{gen_solved_map, Map};
use crate::node::{GreedyPriority, LinearPriority, UniformPriority};
use crate::solver::Solver;

use crate::heuristic::{Euclidian, Hamming, Manhatthan, HRST};
use pancurses::{endwin, Window};
use std::io::Write;

pub struct Executor {
	map: Map,
	solver: Solver,
	window: Window,
}

impl Executor {
	pub fn new(map: Map, heuristic: Heuristics, window: Window) -> Self {
		let solved_map = gen_solved_map(map.size as usize);
		let heuristic = match heuristic {
			Heuristics::Manhatthan => HRST::Manhatthan(Manhatthan::new(&solved_map, map.size)),
			Heuristics::Hamming => HRST::Hamming(Hamming::new(&solved_map, map.size)),
			Heuristics::Euclidian => HRST::Euclidian(Euclidian::new(&solved_map, map.size)),
		};
		let solver = Solver::new(&solved_map, heuristic);

		Executor {
			solver,
			map,
			window,
		}
	}

	pub fn run(&self, priority: Priorities) -> std::io::Result<()> {
		if self.solver.is_solvable(&self.map) {
			let solution = match priority {
				Priorities::Linear => self
					.solver
					.solve::<LinearPriority>(self.map.clone(), &self.window),
				Priorities::Uniform => self
					.solver
					.solve::<UniformPriority>(self.map.clone(), &self.window),
				Priorities::Greedy => self
					.solver
					.solve::<GreedyPriority>(self.map.clone(), &self.window),
			};
			endwin();
			for state in solution.states.iter() {
				println!("{:size$}", state, size = self.map.size as usize);
			}
			print!(
				"Found solution with {} moves, time complexity: {}, memory complexity: {}\n\n",
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

#[derive(clap::ArgEnum)]
pub enum Priorities {
	Linear,
	Greedy,
	Uniform,
}

#[derive(clap::ArgEnum)]
pub enum Heuristics {
	Manhatthan,
	Hamming,
	Euclidian,
}
