use crate::map::{fmt_board, Map};
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Point {
	pub x: u16,
	pub y: u16,
}

impl Point {
	pub(crate) fn to_1d(&self, size: u16) -> u16 {
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

#[derive(Clone)]
pub struct State {
	// state of the board
	pub board: Vec<u16>,
	// position of 0 on board
	zero: Point,
}

impl Display for State {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match f.width() {
			Some(width) => fmt_board(&self.board, width, f),
			None => Err(Error),
		}
	}
}

impl Debug for State {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.board.fmt(f)
	}
}

impl State {
	pub fn new(board: Vec<u16>, zero: Point) -> State {
		State { board, zero }
	}

	fn build_child(&self, new_pos: Point, size: u16) -> State {
		let parent_idx = self.zero.to_1d(size);
		let child_idx = new_pos.to_1d(size);

		let mut v: Vec<u16> = self.board.clone();
		v.swap(parent_idx.into(), child_idx.into());
		State::new(v, new_pos)
	}

	pub fn gen_children(&self, size: u16) -> [Option<State>; 4] {
		let children_pos: [Option<Point>; 4] = [
			self.zero.left(),
			self.zero.right(size),
			self.zero.up(),
			self.zero.down(size),
		];

		let children: [Option<State>; 4] =
			children_pos.map(|el| el.map(|p| self.build_child(p, size)));

		children
	}
}

impl From<Map> for State {
	fn from(map: Map) -> Self {
		let zero = Point::from_1d(
			map.board.iter().position(|&r| r == 0).unwrap() as u16,
			map.size,
		);
		State {
			board: map.board,
			zero,
		}
	}
}

impl Eq for State {}

impl PartialEq for State {
	fn eq(&self, other: &Self) -> bool {
		self.board == other.board
	}
}

impl Hash for State {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.board.hash(state)
	}
}

#[cfg(test)]
mod tests {
	use super::{Point, State};
	use crate::map::Map;

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
		let s = State::from(Map {
			board: vec![1, 2, 3, 4, 5, 6, 7, 0, 8],
			size: 3,
		});
		assert_eq!(s.zero.to_1d(3), 7);
		let s = State::from(Map {
			board: vec![0, 2, 3, 4, 5, 6, 7, 1, 8],
			size: 3,
		});
		assert_eq!(s.zero.to_1d(3), 0);
		let s = State::from(Map {
			board: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
			size: 3,
		});
		assert_eq!(s.zero.to_1d(3), 8);
	}

	#[test]
	fn state_build_child() {
		let s = State::from(Map {
			board: vec![1, 2, 3, 4, 5, 6, 7, 0, 8],
			size: 3,
		});
		let s2 = s.build_child(&Point { x: 0, y: 0 }, 3);
		assert_eq!(s2.zero.to_1d(3), 0);
		assert_eq!(s2.board[s2.zero.to_1d(3) as usize], 0);
		assert_eq!(s2.board[7], 1);
	}

	#[test]
	#[should_panic]
	fn state_build_child_panic() {
		let s = State::from(Map {
			board: vec![1, 2, 3, 4, 5, 6, 7, 0, 8],
			size: 3,
		});
		let _s2 = s.build_child(&Point { x: 10, y: 10 }, 3);
	}
}
