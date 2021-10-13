use crate::state::State;
use crate::state::Point;
use crate::map::Map;

pub trait Heuristic {
    fn new(solved_map: &Map, size: u16) -> Self;
    fn compute_score(&self, state: &State) -> u16;
}

pub struct HMannhathan {
    solved_table: Vec<Point>,
    size: u16
}

impl Heuristic for HMannhathan {
    fn new (solved_map: &Map, size: u16) -> Self {
        let mut solved_table: Vec<Point> =
            vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

        for (i, item) in solved_map.board.iter().enumerate() {
            let p = Point::from_1d(i as u16, solved_map.size);
            solved_table[*item as usize] = p;
        }

        HMannhathan { solved_table, size }
    }

    fn compute_score(&self, state: &State) -> u16 {
        let mut score: u16 = 0;
        for (i, item) in state.board.iter().enumerate() {
            let point = Point::from_1d(i as u16, self.size);
            let dist = Point::manhatthan_dist(&point, &self.solved_table[*item as usize]);
            score += dist;
        }
        score
    }
}

pub struct HHamming {
    solved_table: Vec<u16>,
}

impl Heuristic for HHamming {
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