#[derive(Debug, Clone)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn to_1d(&self, size: u16) -> u16 {
        self.x * size + self.y
    }

    pub fn from_1d(x: u16, size: u16) -> Point {
        Point {
            x: x / size,
            y: x % size,
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

    fn right(&self, size: u16) -> Option<Point> {
        if self.x >= size - 1 {
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

    fn down(&self, size: u16) -> Option<Point> {
        if self.y >= size - 1 {
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
    fn build_state(board: Vec<u16>, size: u16) -> State {
        let zero = Point::from_1d(board.iter().position(|&r| r == 0).unwrap() as u16, size);
        State { board, pos: zero }
    }

    fn build_child(&self, new_pos: &Point, size: u16) -> State {
        let parent_idx = self.pos.to_1d(size);
        let child_idx = new_pos.to_1d(size);

        let mut v: Vec<u16> = self.board.clone();
        v.swap(parent_idx.into(), child_idx.into());
        State::build_state(v, size)
    }

    pub fn gen_children(&self, size: u16) -> [Option<State>; 4] {
        let children_pos: [Option<Point>; 4] = [
            self.pos.left(),
            self.pos.right(size),
            self.pos.up(),
            self.pos.down(size),
        ];

        let children: [Option<State>; 4] = children_pos.map(|el| match el {
            Some(p) => Some(self.build_child(&p, size)),
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
    use super::{Point, State};

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
        let p = Point { x: 1, y: 1 };
        let p = p.right(3).unwrap();
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 1);
        let p = p.right(3);
        match p {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn points_up() {
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
        let p = Point { x: 1, y: 1 };
        let p = p.down(3).unwrap();
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
        let p = p.down(3);
        match p {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn state_build_state() {
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 0, 8], 3);
        assert_eq!(s.pos.to_1d(3), 7);
        let s = State::build_state(vec![0, 2, 3, 4, 5, 6, 7, 1, 8], 3);
        assert_eq!(s.pos.to_1d(3), 0);
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 8, 0], 3);
        assert_eq!(s.pos.to_1d(3), 8);
    }

    #[test]
    fn state_build_child() {
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 0, 8], 3);
        let s2 = s.build_child(&Point { x: 0, y: 0 }, 3);
        assert_eq!(s2.pos.to_1d(3), 0);
        assert_eq!(s2.board[s2.pos.to_1d(3) as usize], 0);
        assert_eq!(s2.board[7], 1);
    }

    #[test]
    #[should_panic]
    fn state_build_child_panic() {
        let s = State::build_state(vec![1, 2, 3, 4, 5, 6, 7, 0, 8], 3);
        let _s2 = s.build_child(&Point { x: 10, y: 10 }, 3);
    }
}
