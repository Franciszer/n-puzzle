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
6
19 27 20 11 30 22
33  7  2 16 17 24
25 28 34 32  3 35
 8  5 31 18 13 14
 4  0  9 29 26 21
 6  1 10 23 15 12
";
	let (_, (size, board)) = parser::parse_map(input)?;
	let executor = Executor::new(parser::validate_map(size, board)?);
	executor.run();
	Ok(())
}
