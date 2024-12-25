use std::io::BufRead;

mod board;
pub mod cmove;
pub mod piece;

fn main() {
    let board = board::Board::default();
    println!("\x1b[2J\x1b[H");
    println!("{}", board);
    let mut stdin = std::io::BufReader::new(std::io::stdin());
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input);
        println!("\x1b[2J\x1b[H");
        println!("{}", board);
        println!("move string: {input}");
        println!("move: {:?}", input.parse::<cmove::Move>());
    }
}
