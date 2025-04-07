use std::fs::write;
use std::io::BufRead;

use chlang::{
    board::{Board, GameState, Player},
    cmove::Move,
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
                    GameState::Win(Player::White) => scores[i] += 2,
                    GameState::Win(Player::Black) => scores[j] += 2,
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
                search_depth: 4,
                cache: FxHashMap::default(),
            });
            black_player = User::Bot(Bot {
                evaluator: Box::new(evaluator_0::Evaluator::default()),
                search_depth: 4,
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

    let mut board = Board::default();

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
            GameState::Draw => {
                #[cfg(not(feature = "compare"))]
                println!("DRAW");
                #[cfg(feature = "compare")]
                println!("0");
                break;
            }
            GameState::Win(Player::White) => {
                #[cfg(not(feature = "compare"))]
                println!("White Wins");
                #[cfg(feature = "compare")]
                println!("1");
                break;
            }
            GameState::Win(Player::Black) => {
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
