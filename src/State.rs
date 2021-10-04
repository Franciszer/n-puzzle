use std::convert::TryInto;
use once_cell::sync::OnceCell;

static SIZE: OnceCell<u16> = OnceCell::new();
pub static BOARD_SIZE: OnceCell<u16> = OnceCell::new();

SIZE.set(3);
// BOARD_SIZE = 9;
// TODO make State.size and State.board_size static

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    #[inline]
    fn to_1d(&self, size: u16) -> u16 {
        self.x * size + self.y
    }

    fn to_2d(idx: u16, size: u16) -> Point {
        Point {
            x: idx / size,
            y: idx % size,
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
        if self.x >= SIZE.get().unwrap() - 1 {
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
        if self.y >= SIZE.get().unwrap() - 1 {
            None
        } else {
            Some(Point {
                x: self.x,
                y: self.y + 1,
            })
        }
    }
}

struct State {
    board: Vec<u16>,
    // state of the board
    pos: Point,
    // position of 0 on board
    size: u16,
    // size one dimension of the board, needs static
    board_size: u16,     // size of the board, needs static
}

impl State {
    fn build_state(board: Vec<u16>, size: u16) -> State {
        let zero = Point::to_2d(board.iter().position(|&r| r == 0).unwrap().try_into().unwrap(), size);
        State {
            board,
            pos: zero,
            size,
            board_size: size * size,
        }
    }

    fn build_child(&self, new_pos: &Point) -> State {
        let parent_idx = self.pos.to_1d(self.size);
        let child_idx = new_pos.to_1d(self.size);

        let mut v: Vec<u16> = self.board.clone();
        v.swap(parent_idx.into(), child_idx.into());
        State::build_state(v, self.size)
    }

    pub fn gen_children(&self) -> [Option<State>; 4] {
        let children_pos: [Option<Point>; 4] = [self.pos.left(), self.pos.right(), self.pos.up(), self.pos.down()];

        let children: [Option<State>; 4] = children_pos.map(|el| {
            match el {
                Some(p) => Some(self.build_child(&p)),
                None => None
            }
        });

        children
    }
}
