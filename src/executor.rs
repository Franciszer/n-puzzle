use crate::heuristic::{Euclidian, Hamming, Manhatthan, HRST};
use crate::map::{gen_solved_map, Map};
use crate::node::{GreedyPriority, LinearPriority, UniformPriority};
use crate::solver::{Solution, Solver};
use crate::state::State;
use pancurses::Window;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

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

	pub fn run(
		&self,
		priority: Priorities,
		replay: Option<PathBuf>,
		skip: bool,
	) -> Result<(), Box<dyn Error>> {
		if self.solver.is_solvable(&self.map) {
			let solution = Solution::<State>::from(self.solve(priority));
			if !skip {
				solution.print(&self.window);
			}
			if let Some(mut path) = replay {
				path.set_extension("replay");
				let file = File::create(path)?;
				bincode::serialize_into(file, &solution)?;
			}
			Ok(())
		} else {
			println!("Puzzle is not solvable !");
			Ok(())
		}
	}

	fn solve(&self, priority: Priorities) -> Solution<Rc<State>> {
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
		solution
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
