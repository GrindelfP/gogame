use crate::board::Board;

mod board;

fn main() {
    let mut board: Board = Board::new(2, 2);
    println!("{board:#?}");
    board.update_at((0, 0), 1);
    println!("{board:#?}");
}
