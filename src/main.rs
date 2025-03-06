use board::Player;
use board::{GameState, Position};
use cmove::Move;
use evaluators::evaluator_0;
use parse::parse;
use piece::Piece;
use rustc_hash::FxHashMap;
use std::fs::{self, write, File};
use std::io::{stdin, BufRead, Read, Write};
use std::path::PathBuf;
use tree_evaluator::Bot;

mod board;
pub mod cmove;
pub mod evaluators;

pub mod game;

// #[cfg(features = "bench")]
pub mod bench;

mod parse;
pub mod piece;
mod train;
pub mod tree_evaluator;

enum User {
    Human,
    Bot(tree_evaluator::Bot),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    Only for development, shows a stacktrace on stack overflows
    (added since I accidentaly ifinitely called a recursive function.
    Solved by adding a boolean flag to the get_pseudo_legal_king_moves method)
    */
    //unsafe { backtrace_on_stack_overflow::enable() };

    let mut a = std::env::args();

    // skip name of program
    a.next();

    let mut white_player = parse::parse(&mut a)?;

    let mut black_player = parse::parse(&mut a)?;

    #[cfg(feature = "gui")]
    {
        let mut app = game::Game {
            white_player,
            black_player,
            ..Default::default()
        };

        return Ok(game::run(&mut app));
    }

    #[cfg(feature = "compare")]
    {
        let mut stdin = std::io::BufReader::new(std::io::stdin());

        // get output file
        println!("Ouput ? ");
        let mut out_file_path = String::new();
        stdin.read_line(&mut out_file_path).unwrap();
        out_file_path = out_file_path.trim().to_string();
        let out_file_path = PathBuf::from(out_file_path);

        // Benchmark
        let result = bench::run(&mut white_player, &mut black_player);

        write(
            out_file_path,
            &format!(
                "Bot 1 wins: {}\nBot 2 wins: {}\nDraws: {}",
                result[0], result[1], result[2]
            ),
        )?;

        return Ok(());
    }

    #[cfg(feature = "train")]
    {
        let mut stdin = std::io::BufReader::new(std::io::stdin());

        println!("Load checkpoint? (type no to start from default)");
        let mut checkpoint_path = String::new();
        stdin.read_line(&mut checkpoint_path).unwrap();
        checkpoint_path = checkpoint_path.trim().to_string();

        if &checkpoint_path != "no" {
            let mut checkpoint_file = File::open(&checkpoint_path)?;
            let mut checkpoint_content = String::new();
            checkpoint_file.read_to_string(&mut checkpoint_content);
            checkpoint_content = checkpoint_content.trim().to_string();

            let mut checkpoint_iter = checkpoint_content.split(" ").map(|x| x.to_owned());

            white_player = parse(&mut checkpoint_iter)?;
            black_player = parse(&mut checkpoint_iter)?;
        } else {
            white_player = User::Bot(Bot {
                evaluator: Box::new(evaluator_0::Evaluator::default()),
                search_depth: 2,
                cache: FxHashMap::default(),
            });
            black_player = User::Bot(Bot {
                evaluator: Box::new(evaluator_0::Evaluator::default()),
                search_depth: 2,
                cache: FxHashMap::default(),
            });
        }

        println!("Save to checkpoint?");
        let mut checkpoint_path = String::new();
        stdin.read_line(&mut checkpoint_path).unwrap();
        checkpoint_path = checkpoint_path.trim().to_string();

        let (mut b1, mut b2) = match (white_player, black_player) {
            (User::Bot(b1), User::Bot(b2)) => (b1, b2),
            _ => Err("Can't train with human bots")?,
        };

        train::train(b1, b2, checkpoint_path);

        return Ok(());
    }

    let mut board = board::Board::default();

    let mut stdin = std::io::BufReader::new(std::io::stdin());
    println!("\x1b[2J\x1b[H");
    println!("{}", board);

    loop {
        match board.turn {
            Player::White => match white_player {
                User::Human => {
                    let mut input = String::new();
                    stdin.read_line(&mut input);
                    if input.trim() == "u" {
                        board.unmake_last();
                    } else {
                        let cmove: Result<Move, &str> = input.parse();

                        if cmove.is_ok() {
                            board.make_move(&cmove.clone().unwrap());
                        }
                    }
                }
                User::Bot(ref mut b) => {
                    let cmove = b.find_best_move(&mut board);
                    if let Some(m) = cmove {
                        board.make_move(&m);
                    }
                }
            },
            Player::Black => match black_player {
                User::Human => {
                    let mut input = String::new();
                    stdin.read_line(&mut input);
                    if input.trim() == "u" {
                        board.unmake_last();
                    } else {
                        let cmove: Result<Move, &str> = input.parse();

                        if cmove.is_ok() {
                            board.make_move(&cmove.clone().unwrap());
                        }
                    }
                }
                User::Bot(ref mut b) => {
                    let cmove = b.find_best_move(&mut board);
                    if let Some(m) = cmove {
                        board.make_move(&m);
                    }
                }
            },
        }

        println!("\x1b[2J\x1b[H");
        println!("{}", board);

        match board.get_game_state() {
            board::GameState::Draw => {
                #[cfg(not(feature = "compare"))]
                println!("DRAW");
                #[cfg(feature = "compare")]
                println!("0");
                break;
            }
            board::GameState::Win(board::Player::White) => {
                #[cfg(not(feature = "compare"))]
                println!("White Wins");
                #[cfg(feature = "compare")]
                println!("1");
                break;
            }
            board::GameState::Win(board::Player::Black) => {
                #[cfg(not(feature = "compare"))]
                println!("Black Wins");
                #[cfg(feature = "compare")]
                println!("-1");
                break;
            }
            _ => (),
        }
    }
    Ok(())
}
