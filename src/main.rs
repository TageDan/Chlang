use std::io::BufRead;

use cmove::Move;

mod board;
pub mod cmove;
pub mod piece;

fn main() {
    /* Only for development, shows a stacktrace on stack overflows
    (added since I accidentaly ifinitely called a recursive function.
    Solved by adding a boolean flag to the get_pseudo_legal_king_moves method)
    */
    unsafe { backtrace_on_stack_overflow::enable() };
    let mut board = board::Board::default();
    println!("\x1b[2J\x1b[H");
    println!("{}", board);
    let mut stdin = std::io::BufReader::new(std::io::stdin());
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input);
        println!("\x1b[2J\x1b[H");
        if input.trim() == "u" {
            board.unmake_last();
        } else {
            let cmove: Result<Move, &str> = input.parse();

            if cmove.is_ok() {
                let res = board.make_move(&cmove.clone().unwrap());
                if res.is_err() {
                    println!("error making move: {:?}", res);
                }
            }
        }

        println!("{}", board);
        match board.get_game_state() {
            board::GameState::Draw => {
                println!("DRAW");
                break;
            }
            board::GameState::Win(board::Player::White) => {
                println!("White Wins");
                break;
            }
            board::GameState::Win(board::Player::Black) => {
                println!("Black Wins");
                break;
            }
            _ => (),
        }
    }
}
