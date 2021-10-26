use ahash::AHashSet;
use pancurses::Window;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use crate::heuristic::{Heuristic, HRST};
use crate::map::{gen_solved_map, Map};
use crate::node::{Node, Priority};
use crate::state::Point;
use crate::state::State;

pub struct Solver {
	heuristic: HRST,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution<T> {
	pub states: Vec<T>,
	pub size: u16,
	// Number of states opened
	pub time: usize,
	// Number of states stored
	pub memory: usize,
}

impl From<Solution<Rc<State>>> for Solution<State> {
	fn from(to_unwrap: Solution<Rc<State>>) -> Self {
		Self {
			states: to_unwrap
				.states
				.into_iter()
				.map(|e| Rc::<State>::try_unwrap(e).unwrap())
				.collect(),
			size: to_unwrap.size,
			time: to_unwrap.time,
			memory: to_unwrap.memory,
		}
	}
}

impl Solver {
	pub fn new(solved_map: &Map, heuristic: HRST) -> Self {
		let mut solved_table: Vec<Point> =
			vec![Point::from_1d(0, solved_map.size); solved_map.board.len()];

		for (i, item) in solved_map.board.iter().enumerate() {
			let p = Point::from_1d(i as u16, solved_map.size);
			solved_table[*item as usize] = p;
		}

		Self { heuristic }
	}
}

impl Solver {
	fn get_inv_count(board: &Vec<u16>) -> u16 {
		let mut count: u16 = 0;

		for i in 0..(board.len() - 1) {
			for j in i + 1..board.len() {
				let left = board[i];
				let right = board[j];

				if  right > left && right != 0 && left != 0 {
					count += 1;
				}
			}
		}

		count
	}

	pub fn is_solvable(&self, map: &Map) -> bool {
		let solved_map = gen_solved_map(map.size as usize);
		let mut inv_count = Self::get_inv_count(&map.board);
		let mut solved_inv_count = Self::get_inv_count(&solved_map.board);

		if map.size.is_even() == true {
			let zero = map.board.iter().position(|&r| r == 0).unwrap() as u16;
			let solved_zero = solved_map.board.iter().position(|&r| r == 0).unwrap() as u16;

			inv_count += zero / map.size;
			solved_inv_count += solved_zero / map.size;
		}

		inv_count.is_even() == solved_inv_count.is_even()
	}

	pub fn solve<P: Priority + Ord>(&self, map: Map, window: &Window) -> Solution<Rc<State>> {
		let size = map.size;
		let root = Rc::new(State::from(map));

		let mut best_score = self.heuristic.compute_score(&root);
		let root_node = Node {
			parent: None,
			state: root.clone(),
			moves: 0,
		};

		let mut nodes = Vec::new();
		let mut states_set: AHashSet<Rc<State>> = AHashSet::new();
		let mut queue: BinaryHeap<P> = BinaryHeap::new();

		nodes.push(root_node);
		queue.push(P::new(0, best_score, 0));
		states_set.insert(root);

		let mut last_print = Instant::now();

		let mut i: usize = 0;
		loop {
			let node_index = queue.pop().unwrap().get_index();
			let state = nodes[node_index].state.clone();
			let moves = nodes[node_index].moves;

			if Instant::now().duration_since(last_print) > Duration::from_secs(1) {
				last_print = Instant::now();
				window.clear();
				window.printw(format!(
					"Distinct: {:9}, Iteration: {:9}, Score: {:3}, Moves: {}\n",
					states_set.len(),
					i,
					best_score,
					moves
				));
				let index = queue.peek().unwrap().get_index();
				window.printw(format!(
					"{}",
					Map {
						size,
						board: nodes[index].state.board.clone()
					}
				));
				window.refresh();
			}

			for child in state.gen_children(size) {
				if let Some(state) = child {
					let state = Rc::new(state);
					i += 1;
					if states_set.insert(state.clone()) {
						let score = self.heuristic.compute_score(&state);
						let new_node = Node {
							parent: Some(node_index),
							state,
							moves: moves + 1,
						};
						if score == 0 {
							return Solution {
								states: new_node.collect_parents(&nodes),
								size,
								time: i,
								memory: states_set.len(),
							};
						}
						queue.push(P::new(nodes.len(), score, new_node.moves));
						if score < best_score {
							best_score = score;
						}
						nodes.push(new_node);
					}
				}
			}
		}
	}
}

trait Oddness {
	fn is_odd(&self) -> bool;
	fn is_even(&self) -> bool;
}

impl Oddness for std::primitive::u16 {
	#[inline]
	fn is_odd(&self) -> bool {
		self & 1 == 1
	}

	#[inline]
	fn is_even(&self) -> bool {
		self & 1 == 0
	}
}

impl Solution<State> {
	pub fn print(&self, window: &Window) {
		// TODO: Skip, Display very large
		let interval = min(
			Duration::from_secs(20) / self.states.len() as u32,
			Duration::from_millis(250),
		);
		let mut last_print;
		for state in self.states.iter() {
			last_print = Instant::now();
			window.clear();
			window.printw(format!(
				"Found solution with {} moves, time complexity: {}, memory complexity: {}\n\n",
				self.states.len(),
				self.time,
				self.memory
			));
			window.printw(format!("{:size$}", state, size = self.size as usize));
			window.refresh();
			if interval > last_print.elapsed() {
				thread::sleep(interval - last_print.elapsed());
			}
		}
		window.mvprintw(window.get_max_y() - 1, 0, "Press any key to continue...");
		window.getch();
	}
}

// #[cfg(test)]
// mod tests {
// 	use crate::map::*;
//
// 	use super::*;
//
// 	#[test]
// 	fn evaluator_build() {
// 		let solved = gen_solved_map(5);
// 		let e = Solver::new(&solved);
// 		println!("{:?}", solved);
// 		println!("len:{} {:?}", e.solved_table.len(), e.solved_table);
// 	}
//
// 	#[test]
// 	fn evaluate_unsolved() {
// 		let mut solved = gen_solved_map(5);
// 		let e = Solver::new(&solved);
//
// 		println!("{:?}", solved);
// 		solved.board.swap(0, 1);
// 		let s = State::from(solved);
// 		assert_eq!(e.compute_score(&s), 2);
//
// 		let mut solved = gen_solved_map(5);
// 		let e = Solver::new(&solved);
//
// 		println!("{:?}", solved);
// 		solved.board.swap(0, 1);
// 		solved.board.swap(5, 22);
// 		solved.board.swap(3, 21);
// 		let s = State::from(solved);
// 		assert_eq!(e.compute_score(&s), 24);
// 	}
//
// 	#[test]
// 	fn evaluate_solved() {
// 		for i in 3..16 {
// 			let solved = gen_solved_map(i);
// 			let e = Solver::new(&solved);
// 			let s = State::from(solved);
// 			assert_eq!(e.compute_score(&s), 0);
// 		}
// 	}
// }
