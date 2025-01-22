use std::borrow::Borrow;

use crate::{
    board::{Board, GameState, Player},
    cmove::Move,
};

pub trait Eval {
    fn evaluate(&self, board: &mut Board) -> isize;
}

pub fn eval(board: &mut Board, depth: u8, evaluator: &Box<dyn Eval>, parent_best: isize) -> isize {
    match board.get_game_state() {
        GameState::Draw => return 0,
        GameState::Playing => (),
        GameState::Win(Player::White) => return isize::MAX,
        GameState::Win(Player::Black) => return isize::MIN,
    }

    if depth == 0 {
        return evaluator.evaluate(board);
    }

    let mut pseudo_legal_moves = board.get_pseudo_legal_moves();

    pseudo_legal_moves.sort_unstable_by_key(|cmove| {
        if board.piece_type(&cmove.to()).is_some() {
            -100
        } else {
            0
        }
    });

    match board.turn {
        Player::White => {
            let mut best = isize::MIN;
            for (i, cmove) in pseudo_legal_moves.into_iter().enumerate() {
                if board.make_move(&cmove).is_ok() {
                    best = best.max(eval(board, depth - 1, evaluator, best));
                    if best >= parent_best {
                        board.unmake_last();
                        return best;
                    }
                    board.unmake_last();
                }
            }
            return best;
        }
        Player::Black => {
            let mut best = isize::MAX;
            for (i, cmove) in pseudo_legal_moves.into_iter().enumerate() {
                if board.make_move(&cmove).is_ok() {
                    best = best.min(eval(board, depth - 1, evaluator, best));
                    if best <= parent_best {
                        board.unmake_last();
                        return best;
                    }
                    board.unmake_last();
                }
            }
            return best;
        }
    }
}

pub struct Bot {
    pub evaluator: Box<dyn Eval>,
    pub search_depth: u8,
}

impl Bot {
    pub fn find_best_move(&self, board: &mut Board) -> Option<Move> {
        let mut pseudo_legal_moves = board.get_pseudo_legal_moves();

        pseudo_legal_moves.sort_unstable_by_key(|cmove| {
            if board.piece_type(&cmove.to()).is_some() {
                -100
            } else {
                0
            }
        });

        match board.turn {
            Player::White => {
                let mut best_move = (None, isize::MIN);
                for cmove in pseudo_legal_moves {
                    if board.make_move(&cmove).is_ok() {
                        let val = eval(board, self.search_depth - 1, &self.evaluator, best_move.1);
                        if val >= best_move.1 {
                            best_move = (Some(cmove), val);
                        }
                        board.unmake_last();
                    }
                }
                return best_move.0;
            }
            Player::Black => {
                let mut best_move = (None, isize::MAX);
                for cmove in pseudo_legal_moves {
                    if board.make_move(&cmove).is_ok() {
                        let val = eval(
                            board,
                            self.search_depth - 1,
                            self.evaluator.borrow(),
                            best_move.1,
                        );
                        if val <= best_move.1 {
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
