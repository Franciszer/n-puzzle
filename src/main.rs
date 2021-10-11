#![allow(dead_code)] // Remove when project is somewhat finished
use executor::Executor;
use std::error::Error;

mod solver;
mod executor;
mod map;
mod node;
mod parser;
mod state;
mod evaluator;

fn main() -> Result<(), Box<dyn Error>> {
	let input = "
# This puzzle is solvable
4
 2 12 13 10
14  6 15  1
 3  0 11  8
 4  9  7  5
";
	let (_, (size, board)) = parser::parse_map(input)?;
	let executor = Executor::new(parser::validate_map(size, board)?);
	executor.run();
	Ok(())
}
