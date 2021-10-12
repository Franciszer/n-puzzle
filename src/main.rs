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
30  3 10 29  7 19
 9 20 32  5 15 12
24 34 18 14 16  0
22 27 35 11  1 17
25 23 31 33  8 13
28  4  2 26 21  6
";
	let (_, (size, board)) = parser::parse_map(input)?;
	let executor = Executor::new(parser::validate_map(size, board)?);
	executor.run();
	Ok(())
}
