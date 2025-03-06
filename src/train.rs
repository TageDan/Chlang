use std::fs::write;

use crate::{
    board::{Board, GameState},
    game,
    tree_evaluator::Bot,
    User,
};

pub fn train(mut b1: Bot, mut b2: Bot, checkpoint_path: String) {
    let mut contenders = [b1.bot_clone(), b2.bot_clone(), b1.modified(), b2.modified()];
    loop {
        let mut scores: [usize; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                let mut wp = User::Bot(contenders[i].bot_clone());
                let mut bp = User::Bot(contenders[j].bot_clone());
                match game::run(&mut wp, &mut bp) {
                    GameState::Win(crate::board::Player::White) => scores[i] += 2,
                    GameState::Win(crate::board::Player::Black) => scores[j] += 2,
                    GameState::Draw => {
                        scores[i] += 1;
                        scores[j] += 1;
                    }
                    GameState::Playing => (),
                }
                let mut wp = User::Bot(contenders[j].bot_clone());
                let mut bp = User::Bot(contenders[i].bot_clone());
                match game::run(&mut wp, &mut bp) {
                    GameState::Win(crate::board::Player::White) => scores[j] += 2,
                    GameState::Win(crate::board::Player::Black) => scores[i] += 2,
                    GameState::Draw => {
                        scores[i] += 1;
                        scores[j] += 1;
                    }
                    GameState::Playing => (),
                }
            }
        }
        let max_index = scores
            .iter()
            .enumerate()
            .fold((0_usize, 0_usize), |acc, x| {
                if *x.1 > acc.1 {
                    (x.0, *x.1)
                } else {
                    acc
                }
            });

        let second_best = scores
            .iter()
            .enumerate()
            .fold((0_usize, 0_usize), |acc, x| {
                if *x.1 > acc.1 && x.0 != max_index.0 {
                    (x.0, *x.1)
                } else {
                    acc
                }
            });

        let best_b = contenders[max_index.0].bot_clone();
        let second_b = contenders[second_best.0].bot_clone();
        contenders[0] = best_b.bot_clone();
        contenders[1] = second_b.bot_clone();
        contenders[2] = best_b.modified();
        contenders[3] = second_b.modified();

        println!("writing to checkpoint");
        println!(
            "best bot bytes: {:?}",
            best_b.evaluator.string_rep().as_bytes()
        );
        println!("best bot str: {:?}", best_b.evaluator.string_rep());
        write(
            &checkpoint_path,
            best_b.evaluator.string_rep() + &second_b.evaluator.string_rep(),
        )
        .unwrap();
        println!("wrote to checkpoint")
    }
}
