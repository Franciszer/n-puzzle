#![allow(dead_code)] // Remove when project is somewhat finished
use std::error::Error;
use std::fmt::{Debug, Formatter};

mod parser;
mod state;

pub struct Map {
    size: u16,
    board: Vec<u16>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = match self.board.iter().max() {
            Some(w) => w.to_string().len(),
            None => return Ok(()),
        };
        for line in self.board.chunks_exact(self.size as usize) {
            for (i, e) in line.iter().enumerate() {
                if i == self.size as usize - 1 {
                    write!(f, "{:width$}", e, width = width)?;
                } else {
                    write!(f, "{:width$} ", e, width = width)?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
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
    print!("{:?}", map);
    state::MAP.set(map).unwrap();
    Ok(())
}
