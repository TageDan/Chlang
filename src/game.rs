use crate::{
    board::{self, GameState, Player},
    User,
};

#[cfg(not(feature = "gui"))]
pub fn run(white_player: &mut User, black_player: &mut User) -> GameState {
    let mut board = board::Board::default();
    loop {
        match board.turn {
            Player::White => match white_player {
                User::Bot(ref mut b) => {
                    let cmove = b.find_best_move(&mut board);
                    if let Some(m) = cmove {
                        board.make_move(&m);
                    }
                }
                _ => {
                    panic!("should not have humans")
                }
            },
            Player::Black => match black_player {
                User::Bot(ref mut b) => {
                    let cmove = b.find_best_move(&mut board);
                    if let Some(m) = cmove {
                        board.make_move(&m);
                    }
                }
                _ => panic!("should not have humans"),
            },
        }

        match board.get_game_state() {
            GameState::Playing => (),
            _ => return board.get_game_state(),
        }
    }
}
