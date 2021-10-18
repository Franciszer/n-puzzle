use crate::map::Map;
extern crate rand;
use self::rand::thread_rng;
use rand::seq::SliceRandom;

// generates a random map
pub struct Generator {
	size: usize,
	board_size: usize,
}

impl Generator {
	pub fn new(size: usize) -> Self {
		Self {
			size,
			board_size: size.pow(2),
		}
	}

	pub fn generate(&self) -> Map {
		let mut board: Vec<u16> = (0..self.board_size as u16).collect();

		board.shuffle(&mut thread_rng());

		Map {
			size: self.size as u16,
			board,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn random_board() {
		let g = Generator::new(3);

		let map = g.generate();

		println!("{:?}", map);
	}
}
