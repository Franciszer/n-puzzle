use clap::Clap;
use executor::Executor;
use executor::Heuristics;
use executor::Priorities;
use pancurses::{endwin, initscr};
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};
use std::fs::File;
use crate::solver::Solution;
use crate::state::State;

mod executor;
mod heuristic;
mod map;
mod node;
mod parser;
mod solver;
mod state;
mod generator;

#[derive(Clap)]
struct Opts {
	/// Search function to use
	#[clap(short, long, arg_enum, default_value = "linear")]
	search: Priorities,
	/// Heuristic function to use
	#[clap(short, long, arg_enum, default_value = "manhatthan")]
	heuristic: Heuristics,
	/// Puzzle to solve
	map: Option<PathBuf>,
	/// Save the solution for replay
	#[clap(long, parse(from_os_str))]
	save: Option<PathBuf>,
	/// Do not print solution
	#[clap(long)]
	skip: bool,
	/// Replay solution
	#[clap(short, long,parse(from_os_str))]
	replay: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn Error>> {
	let opts: Opts = Opts::parse();
	if let Some(path) = opts.replay {
		replay(path)
	} else {
		solve(opts)
	}
}

fn solve(opts: Opts) -> Result<(), Box<dyn Error>> {
	let input: String = match opts.map {
		Some(filename) => fs::read_to_string(filename)?,
		None => {
			let mut tmp = String::new();
			io::stdin().read_to_string(&mut tmp)?;
			tmp
		}
	};

	let (_, (size, board)) = parser::parse_map(&input).or(Err("Unable to parse map !"))?;
	let executor = Executor::new(
		parser::validate_map(size, board)?,
		opts.heuristic,
		initscr(),
	);
	executor.run(opts.search, opts.save, opts.skip)?;
	endwin();
	Ok(())
}

fn replay(replay_file: PathBuf) -> Result<(), Box<dyn Error>> {
	let file = File::open(replay_file)?;
	let solution: Solution<State> = bincode::deserialize_from(file)?;
	solution.print(&initscr());
	endwin();
	Ok(())
}
