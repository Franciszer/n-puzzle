use executor::Executor;
use std::error::Error;

mod executor;
mod map;
mod node;
mod parser;
mod solver;
mod state;

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
