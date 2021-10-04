#![allow(dead_code)] // Remove when project is somewhat finished
use std::error::Error;

mod parser;
mod state;

#[derive(Debug)]
pub struct Map {
    size: u16,
    board: Vec<u16>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "4
0  10 5  7
11 14 4  8
1  2  6  13
12 3  15 9
";

    let (_, (size, board)) = parser::parse_map(input)?;
    let map = parser::validate_map(size, board)?;
    println!("{:?}", map);
    state::MAP.set(map).unwrap();
    Ok(())
}
