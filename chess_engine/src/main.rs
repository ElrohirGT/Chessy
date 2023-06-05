use chess_engine::get_starting_board;
use serde_json;

fn main() {
    let board = get_starting_board();

    println!("{}", serde_json::to_string(&board).unwrap());
}
