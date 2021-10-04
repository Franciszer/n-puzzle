use crate::Map;
use once_cell::sync::OnceCell;
use std::convert::TryInto;

pub static MAP: OnceCell<Map> = OnceCell::new();

// TODO make State.size and State.board_size static

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    #[inline]
    fn to_1d(&self) -> u16 {
        self.x * MAP.get().unwrap().size + self.y
    }

    fn to_2d(idx: u16) -> Point {
        Point {
            x: idx / MAP.get().unwrap().size,
            y: idx % MAP.get().unwrap().size,
        }
    }

    fn left(&self) -> Option<Point> {
        if self.x == 0 {
            None
        } else {
            Some(Point {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    fn right(&self) -> Option<Point> {
        if self.x >= MAP.get().unwrap().size - 1 {
            None
        } else {
            Some(Point {
                x: self.x + 1,
                y: self.y,
            })
        }
    }

    fn up(&self) -> Option<Point> {
        if self.y == 0 {
            None
        } else {
            Some(Point {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn down(&self) -> Option<Point> {
        if self.y >= MAP.get().unwrap().size - 1 {
            None
        } else {
            Some(Point {
                x: self.x,
                y: self.y + 1,
            })
        }
    }
}

pub struct State {
    // state of the board
    board: Vec<u16>,
    // position of 0 on board
    pos: Point,
}

impl State {
    fn build_state(board: Vec<u16>) -> State {
        let zero = Point::to_2d(
            board
                .iter()
                .position(|&r| r == 0)
                .unwrap()
                .try_into()
                .unwrap(),
        );
        State { board, pos: zero }
    }

    fn build_child(&self, new_pos: &Point) -> State {
        let parent_idx = self.pos.to_1d();
        let child_idx = new_pos.to_1d();

        let mut v: Vec<u16> = self.board.clone();
        v.swap(parent_idx.into(), child_idx.into());
        State::build_state(v)
    }

    pub fn gen_children(&self) -> [Option<State>; 4] {
        let children_pos: [Option<Point>; 4] = [
            self.pos.left(),
            self.pos.right(),
            self.pos.up(),
            self.pos.down(),
        ];

        let children: [Option<State>; 4] = children_pos.map(|el| match el {
            Some(p) => Some(self.build_child(&p)),
            None => None,
        });

        children
    }

    // WILL BE MOVED TO HEURISTIC
    // fn      get_inv_count(&self) -> u16 {
    //     let mut count: u16 = 0;
    //
    //     for i in 0..(self.board.len() - 1) {
    //         for j in i+1..self.board.len() {
    //
    //             let left = self.board[i];
    //             let right = self.board[j];
    //
    //             if (right != 0 && right > left) {
    //                 count += 1;
    //             }
    //         }
    //     }
    //
    //     count
    // }

    // pub fn is_solvable(&self) -> bool {
    //     let inv_count = self.get_inv_count();
    //
    //     if (SIZE.get().unwrap() & 1 != 0) {
    //         inv_count & 1 == 0
    //     }
    //     else {
    //         let pos: u16 = self.pos.to_1d();
    //
    //         if (pos & 1 != 0) {
    //             inv_count & 1 == 0
    //         }
    //         else {
    //             inv_count & 1 != 0
    //         }
    //     }
    //
    // }
}

#[cfg(test)]
mod tests {
    use crate::state::Point;

    #[test]
    fn points_basic() {
        let p = Point { x: 1, y: 1 };
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 1);
    }

    #[test]
    fn points_left() {
        let p = Point { x: 1, y: 1 };
        let p = p.left().unwrap();
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 1);
        let p = p.left();
        match p {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn points_right() {
        crate::state::SIZE.set(3).unwrap();
        crate::state::BOARD_SIZE.set(9).unwrap();
        let p = Point { x: 1, y: 1 };
        let p = p.right().unwrap();
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 1);
        let p = p.right();
        match p {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn points_up() {
        crate::state::SIZE.set(3).unwrap();
        crate::state::BOARD_SIZE.set(9).unwrap();
        let p = Point { x: 1, y: 1 };
        let p = p.up().unwrap();
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 0);
        let p = p.up();
        match p {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn points_down() {
        crate::state::SIZE.set(3).unwrap();
        crate::state::BOARD_SIZE.set(9).unwrap();
        let p = Point { x: 1, y: 1 };
        let p = p.down().unwrap();
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
        let p = p.down();
        match p {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn state_build_state() {
        use super::*;
        SIZE.set(3).unwrap();
        BOARD_SIZE.set(9).unwrap();
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 0, 8]);
        assert_eq!(s.pos.to_1d(), 7);
        let s = State::build_state(vec![0, 2, 3, 4, 5, 6, 7, 1, 8]);
        assert_eq!(s.pos.to_1d(), 0);
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 8, 0]);
        assert_eq!(s.pos.to_1d(), 8);
    }

    #[test]
    fn state_build_child() {
        use super::*;
        SIZE.set(3).unwrap();
        BOARD_SIZE.set(9).unwrap();
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 0, 8]);
        let s2 = s.build_child(&Point { x: 0, y: 0 });
        assert_eq!(s2.pos.to_1d(), 0);
        assert_eq!(s2.board[s2.pos.to_1d() as usize], 0);
        assert_eq!(s2.board[7], 1);
    }

    #[test]
    #[should_panic]
    fn state_build_child_panic() {
        use super::*;
        SIZE.set(3).unwrap();
        BOARD_SIZE.set(9).unwrap();
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 0, 8]);
        let _s2 = s.build_child(&Point { x: 10, y: 10 });
    }
}
