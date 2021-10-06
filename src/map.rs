use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub struct Map {
    pub size: u16,
    pub board: Vec<u16>,
}

impl Display for Map {
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

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn gen_solved_map(size: usize) -> Map {
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

#[cfg(test)]
mod tests {
    use super::gen_solved_map;
    use super::Map;

    #[test]
    fn gen_map_3() {
        let result = gen_solved_map(3);
        let map = Map {
            size: 3,
            board: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
        };
        assert_eq!(map, result);
    }

    #[test]
    fn gen_map_4() {
        let result = gen_solved_map(4);
        let map = Map {
            size: 4,
            board: vec![1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7],
        };
        assert_eq!(map, result);
    }

    #[test]
    fn gen_map_5() {
        let result = gen_solved_map(5);
        let map = Map {
            size: 5,
            board: vec![
                1, 2, 3, 4, 5, 16, 17, 18, 19, 6, 15, 24, 0, 20, 7, 14, 23, 22, 21, 8, 13, 12, 11,
                10, 9,
            ],
        };
        assert_eq!(map, result);
    }

    #[test]
    fn gen_map_8() {
        let result = gen_solved_map(8);
        let map = Map {
            size: 8,
            board: vec![
                1, 2, 3, 4, 5, 6, 7, 8, 28, 29, 30, 31, 32, 33, 34, 9, 27, 48, 49, 50, 51, 52, 35,
                10, 26, 47, 60, 61, 62, 53, 36, 11, 25, 46, 59, 0, 63, 54, 37, 12, 24, 45, 58, 57,
                56, 55, 38, 13, 23, 44, 43, 42, 41, 40, 39, 14, 22, 21, 20, 19, 18, 17, 16, 15,
            ],
        };
        assert_eq!(map, result);
    }

    #[test]
    fn gen_map_9() {
        let result = gen_solved_map(9);
        let map = Map {
            size: 9,
            board: vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 32, 33, 34, 35, 36, 37, 38, 39, 10, 31, 56, 57, 58, 59,
                60, 61, 40, 11, 30, 55, 72, 73, 74, 75, 62, 41, 12, 29, 54, 71, 80, 0, 76, 63, 42,
                13, 28, 53, 70, 79, 78, 77, 64, 43, 14, 27, 52, 69, 68, 67, 66, 65, 44, 15, 26, 51,
                50, 49, 48, 47, 46, 45, 16, 25, 24, 23, 22, 21, 20, 19, 18, 17,
            ],
        };
        assert_eq!(map, result);
    }
}
