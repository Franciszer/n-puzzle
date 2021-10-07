use crate::map::{gen_solved_map, Map};
use crate::evaluator::Evaluator;
use crate::evaluator::*;

pub struct Executor {
    map: Map,
    evaluator: Evaluator
}

impl Executor {
    pub fn new(map: Map) -> Self {
        let map1 = gen_solved_map(map.size as usize);
        Executor {
            map,
            evaluator: Evaluator::build_evaluator(&map1)
        }
    }

    pub fn run(&self) {
        print!("{}\n", self.map);
    }

    // fn solve
}
