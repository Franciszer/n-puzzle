use executor::Executor;
use std::error::Error;
use std::{env, fs};

mod executor;
mod map;
mod node;
mod parser;
mod solver;
mod state;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	let filename: &str = match args.get(1) {
		Some(s) => s,
		None => "./maps/6x6.map",
	};
	let input = fs::read_to_string(filename)?;
	let (_, (size, board)) = parser::parse_map(&input).or(Err("Unable to parse map !"))?;
	let executor = Executor::new(parser::validate_map(size, board)?);
	executor.run();
	Ok(())
}
