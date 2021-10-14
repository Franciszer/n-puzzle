use crate::executor::Heuristics;
use clap::Clap;
use executor::Executor;
use executor::Priorities;
use std::error::Error;
use std::io::Read;
use std::{fs, io};

mod executor;
mod heuristic;
mod map;
mod node;
mod parser;
mod solver;
mod state;

#[derive(Clap)]
struct Opts {
	#[clap(short, long, arg_enum, default_value = "linear")]
	search: Priorities,
	#[clap(short, long, arg_enum, default_value = "manhatthan")]
	heuristic: Heuristics,
	map: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let opts: Opts = Opts::parse();
	let input: String = match opts.map {
		Some(filename) => fs::read_to_string(filename)?,
		None => {
			let mut tmp = String::new();
			io::stdin().read_to_string(&mut tmp)?;
			tmp
		}
	};

	let (_, (size, board)) = parser::parse_map(&input).or(Err("Unable to parse map !"))?;
	let executor = Executor::new(parser::validate_map(size, board)?, opts.heuristic);
	executor.run(opts.search)?;
	Ok(())
}
