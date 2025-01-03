use std::io::BufRead;

use cmove::Move;

mod board;
pub mod cmove;
pub mod piece;

fn main() {
    let mut board = board::Board::default();
    println!("\x1b[2J\x1b[H");
    println!("{}", board);
    let mut stdin = std::io::BufReader::new(std::io::stdin());
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input);
        let cmove: Result<Move, &str> = input.parse();

        if cmove.is_ok() {
            board.make_move(cmove.clone().unwrap());
        }

        println!("\x1b[2J\x1b[H");
        println!("{}", board);
        println!("move string: {input}");
        println!("move: {:?}", cmove);
    }
}
