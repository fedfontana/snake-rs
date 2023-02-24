use crate::board::Board;

mod snake;
mod board;

fn main() {
    println!("{}", Board::new_with_size(10, 10));
}
