use crate::map::{gen_solved_map, Map};

pub struct Executor {
    map: Map,
    solved_map: Map,
}

impl Executor {
    pub fn new(map: Map) -> Self {
        Executor {
            solved_map: gen_solved_map(map.size as usize),
            map,
        }
    }

    pub fn run(&self) {
        print!("{}\n{}", self.map, self.solved_map);
    }
}
