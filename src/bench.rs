use std::io::BufRead;

use crate::{board, game, User};

#[cfg(feature = "compare")]
pub fn run(b1: &mut User, b2: &mut User) -> [usize; 3] {
    match b1 {
        User::Human => panic!("can't bench with human player"),
        _ => (),
    }
    match b2 {
        User::Human => panic!("can't bench with human player"),
        _ => (),
    }

    let mut stdin = std::io::BufReader::new(std::io::stdin());

    // get iterations
    println!("Iterations ? ");
    let mut iters = String::new();
    stdin.read_line(&mut iters).unwrap();
    iters = iters.trim().to_string();
    let iters = iters.parse::<usize>().unwrap();

    let mut wins = [0, 0, 0];

    for i in 0..(iters * 2) {
        if i % 2 == 0 {
            match game::run(b1, b2) {
                board::GameState::Win(board::Player::White) => wins[0] += 1,
                board::GameState::Win(board::Player::Black) => wins[1] += 1,
                board::GameState::Draw => wins[2] += 1,
                _ => (),
            }
        } else {
            match game::run(b2, b1) {
                board::GameState::Win(board::Player::White) => wins[1] += 1,
                board::GameState::Win(board::Player::Black) => wins[0] += 1,
                board::GameState::Draw => wins[2] += 1,
                _ => (),
            }
        }
    }

    wins
}
