#![allow(dead_code)] // Remove when project is somewhat finished
use executor::Executor;
use std::error::Error;

mod executor;
mod map;
mod parser;
mod state;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "6
0  10 5  7  16 25
11 14 4  8  17 26
1  2  6  13 18 27
12 3  15 9  19 28
20 21 22 23 24 29
30 31 32 33 34 35
";

    let (_, (size, board)) = parser::parse_map(input)?;
    let executor = Executor::new(parser::validate_map(size, board)?);
    executor.run();
    Ok(())
}
