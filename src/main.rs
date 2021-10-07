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
mod node;

fn main() -> Result<(), Box<dyn Error>> {
	let input = "
# This puzzle is solvable
4
10  0 13  7
 1  3  2 11
 6  9 15  5
14  4 12  8
";
	let (_, (size, board)) = parser::parse_map(input)?;
	let executor = Executor::new(parser::validate_map(size, board)?);
	executor.run();
	Ok(())
}
