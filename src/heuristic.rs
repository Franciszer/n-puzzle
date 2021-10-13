use crate::state::State;
use crate::state::Point;
use crate::map::Map;

pub trait Heuristic {
    fn new(solved_map: &Map, size: u16) -> Self;
    fn compute_score(&self, state: &State) -> u16;
}

pub struct Manhatthan {
    solved_table: Vec<Point>,
    size: u16
}

impl Manhatthan {
    pub fn dist(p1: &Point, p2: &Point) -> u16 {
        let x_dist = match p1.x > p2.x {
            true => p1.x - p2.x,
            false => p2.x - p1.x,
        };

        let y_dist = match p1.y > p2.y {
            true => p1.y - p2.y,
            false => p2.y - p1.y,
        };

        x_dist + y_dist
    }
}

impl Heuristic for Manhatthan {
    fn new (solved_map: &Map, size: u16) -> Self {
        let mut solved_table: Vec<Point> =
            vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

        for (i, item) in solved_map.board.iter().enumerate() {
            let p = Point::from_1d(i as u16, solved_map.size);
            solved_table[*item as usize] = p;
        }

        Self { solved_table, size }
    }

    fn compute_score(&self, state: &State) -> u16 {
        let mut score: u16 = 0;
        for (i, item) in state.board.iter().enumerate() {
            let point = Point::from_1d(i as u16, self.size);
            let dist = Self::dist(&point, &self.solved_table[*item as usize]);
            score += dist;
        }
        score
    }
}

pub struct Hamming {
    solved_table: Vec<u16>,
}

impl Heuristic for Hamming {
    fn new (solved_map: &Map, _size: u16) -> Self {
        let mut solved_table: Vec<u16> =
            vec![0; solved_map.board.len()];

        for (i, item) in solved_map.board.iter().enumerate() {
            solved_table[*item as usize] = i as u16;
        }

        Self { solved_table }
    }

    fn compute_score(&self, state: &State) -> u16 {
        let mut score: u16 = 0;
        for (i, item) in state.board.iter().enumerate() {
            if self.solved_table[*item as usize] == i as u16 {
                score += 1;
            }
        }
        state.board.len() as u16 - score
    }
}

pub struct Euclidian {
    solved_table: Vec<Point>,
    size: u16
}

impl Euclidian {
    pub fn dist(p1: &Point, p2: &Point) -> u16 {
        let x_dist = match p1.x > p2.x {
            true => p1.x.pow(2) - p2.x.pow(2),
            false => p2.x.pow(2) - p1.x.pow(2),
        };

        let y_dist = match p1.y.pow(2) > p2.y.pow(2) {
            true => p1.y.pow(2) - p2.y.pow(2),
            false => p2.y.pow(2) - p1.y.pow(2),
        };

        x_dist + y_dist
    }
}

impl Heuristic for Euclidian {
    fn new (solved_map: &Map, size: u16) -> Self {
        let mut solved_table: Vec<Point> =
            vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

        for (i, item) in solved_map.board.iter().enumerate() {
            let p = Point::from_1d(i as u16, solved_map.size);
            solved_table[*item as usize] = p;
        }

        Self { solved_table, size }
    }

    fn compute_score(&self, state: &State) -> u16 {
        let mut score: u16 = 0;
        for (i, item) in state.board.iter().enumerate() {
            let point = Point::from_1d(i as u16, self.size);
            let dist = Self::dist(&point, &self.solved_table[*item as usize]);
            score += dist;
        }
        score
    }
}
