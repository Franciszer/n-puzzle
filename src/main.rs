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

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn gen_solved_map(size: usize) -> Map {
    let mut sizex = size;
    let mut sizey = size;
    let mut board = vec![0; size * size];
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut i: u16 = 1;
    let mut direction = Direction::Right;

    loop {
        let index = y * size + x;
        board[index] = i;
        i += 1;

        loop {
            match direction {
                Direction::Right => {
                    if x == sizex - 1 {
                        direction = Direction::Down;
                    } else {
                        x += 1;
                        break;
                    }
                }
                Direction::Down => {
                    if y == sizey - 1 {
                        direction = Direction::Left;
                    } else {
                        y += 1;
                        break;
                    }
                }
                Direction::Left => {
                    if x == size - sizex {
                        sizey -= 1;
                        direction = Direction::Up;
                    } else {
                        x -= 1;
                        break;
                    }
                }
                Direction::Up => {
                    if y == size - sizey {
                        sizex -= 1;
                        direction = Direction::Right;
                    } else {
                        y -= 1;
                        break;
                    }
                }
            };
        }

        if i as usize == size * size {
            break;
        }
    }

    Map {
        size: size as u16,
        board,
    }
}

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
    let map = parser::validate_map(size, board)?;
    print!("{:?}", gen_solved_map(9));
    state::MAP.set(map).unwrap();
    Ok(())
}
