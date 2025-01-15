use crate::{
    board::{Board, GameState, Player},
    cmove::Move,
};

pub trait Eval {
    fn evaluate(&self, board: &mut Board) -> isize;
}

pub fn eval(board: &mut Board, depth: u8, evaluator: &impl Eval) -> isize {
    if depth == 0 {
        return evaluator.evaluate(board);
    }

    match board.get_game_state() {
        GameState::Draw => return 0,
        GameState::Playing => (),
        GameState::Win(Player::White) => return isize::MAX,
        GameState::Win(Player::Black) => return isize::MIN,
    }

    match board.turn {
        Player::White => {
            let mut best = isize::MIN;
            for cmove in board.get_pseudo_legal_moves() {
                if board.make_move(&cmove).is_ok() {
                    best = best.max(eval(board, depth - 1, evaluator));
                    board.unmake_last();
                }
            }
            return best;
        }
        Player::Black => {
            let mut best = isize::MAX;
            for cmove in board.get_pseudo_legal_moves() {
                if board.make_move(&cmove).is_ok() {
                    best = best.min(eval(board, depth - 1, evaluator));
                    board.unmake_last();
                }
            }
            return best;
        }
    }
}

pub struct Bot<T: Eval> {
    pub evaluator: T,
    pub search_depth: u8,
}

impl<T: Eval> Bot<T> {
    pub fn find_best_move(&self, board: &mut Board) -> Option<Move> {
        match board.turn {
            Player::White => {
                let mut best_move = (None, isize::MIN);
                for cmove in board.get_pseudo_legal_moves() {
                    if board.make_move(&cmove).is_ok() {
                        let val = eval(board, self.search_depth - 1, &self.evaluator);
                        if val > best_move.1 {
                            best_move = (Some(cmove), val);
                        }
                        board.unmake_last();
                    }
                }
                return best_move.0;
            }
            Player::Black => {
                let mut best_move = (None, isize::MAX);
                for cmove in board.get_pseudo_legal_moves() {
                    if board.make_move(&cmove).is_ok() {
                        let val = eval(board, self.search_depth - 1, &self.evaluator);
                        if val < best_move.1 {
                            best_move = (Some(cmove), val);
                        }
                        board.unmake_last();
                    }
                }
                return best_move.0;
            }
        }
    }
}
