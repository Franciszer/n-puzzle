
mod state;

fn main() {
    state::SIZE.set(3).unwrap();
    state::BOARD_SIZE.set(9).unwrap();
}
