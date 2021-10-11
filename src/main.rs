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
3
7 1 2
3 0 6
8 4 5
";
	let (_, (size, board)) = parser::parse_map(input)?;
	let executor = Executor::new(parser::validate_map(size, board)?);
	executor.run();
	Ok(())
}
