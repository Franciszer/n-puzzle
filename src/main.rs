use crate::generator::Generator;
use crate::map::Map;
use crate::solver::Solution;
use crate::state::State;
use clap::Clap;
use clap_num::si_number_range;
use executor::Executor;
use executor::Heuristics;
use executor::Priorities;
use pancurses::{endwin, initscr};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};
use std::ffi::OsStr;
use flate2::read::GzDecoder;

mod executor;
mod generator;
mod heuristic;
mod map;
mod node;
mod parser;
mod solver;
mod state;

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
	/// Generate random map of size <GENERATE> if greater than 0
	#[clap(short, long, parse(try_from_str=generator_size), default_value="2")]
	generate: u16,
	/// Replay solution
	#[clap(short, long, parse(from_os_str))]
	replay: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let opts: Opts = Opts::parse();
	if let Some(path) = opts.replay {
		replay(path)
	} else {
		solve(opts)
	}
}

// 2 serves as the null value
fn generator_size(s: &str) -> Result<u16, String> {
	si_number_range(s, 2, 15)
}

fn get_map(opts: &Opts) -> Result<Map, Box<dyn Error>> {
	if opts.generate > 2 {
		let g = Generator::new(opts.generate as usize);
		Ok(g.generate())
	} else {
		let input: String = match &opts.map {
			Some(filename) => fs::read_to_string(filename)?,
			None => {
				let mut tmp = String::new();
				io::stdin().read_to_string(&mut tmp)?;
				tmp
			}
		};

		let (_, (size, board)) = parser::parse_map(&input).or(Err("Unable to parse map !"))?;
		Ok(parser::validate_map(size, board)?)
	}
}

fn solve(opts: Opts) -> Result<(), Box<dyn Error>> {
	let map = get_map(&opts)?;
	let executor = Executor::new(map, opts.heuristic, initscr());
	executor.run(opts.search, opts.save, opts.skip)?;
	endwin();
	Ok(())
}

fn replay(replay_file: PathBuf) -> Result<(), Box<dyn Error>> {
	let file = File::open(&replay_file)?;
	let solution: Solution<State> = if replay_file.extension() == Some(OsStr::new("gz")) {
		bincode::deserialize_from(GzDecoder::new(file))?
	} else {
		bincode::deserialize_from(file)?
	};
	solution.print(&initscr());
	endwin();
	Ok(())
}
