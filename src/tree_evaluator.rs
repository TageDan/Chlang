use std::{borrow::Borrow, collections::HashMap};

use rand::prelude::*;
use rustc_hash::FxHashMap;

use crate::{
    board::{Board, GameState, KeyStruct, Player},
    cmove::Move,
};

pub trait Eval {
    fn evaluate(&self, board: &mut Board) -> isize;
}

pub fn eval(
    board: &mut Board,
    depth: u8,
    evaluator: &Box<dyn Eval>,
    mut alpha: isize,
    mut beta: isize,
    cache: &mut FxHashMap<KeyStruct, (isize, bool, u8)>,
) -> isize {
    match board.get_game_state() {
        GameState::Draw => return 0,
        GameState::Playing => (),
        GameState::Win(Player::White) => return isize::MAX,
        GameState::Win(Player::Black) => return isize::MIN,
    }
    if let Some((value, fin, max_d)) = cache.get(&board.key()) {
        if depth <= *max_d {
            if *fin {
                return *value;
            } else if *value > beta && board.turn == Player::White {
                return *value;
            } else if *value < alpha && board.turn == Player::Black {
                return *value;
            }
        }
    }

    if depth == 0 {
        let val = evaluator.evaluate(board);
        let e = cache.entry(board.key()).or_insert((0, true, depth));

        e.0 = val;
        return val;
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
            for cmove in pseudo_legal_moves.into_iter() {
                if board.make_move(&cmove).is_ok() {
                    best = best.max(eval(board, depth - 1, evaluator, alpha, beta, cache));
                    if best >= beta {
                        board.unmake_last();
                        break;
                    }
                    alpha = alpha.max(best);
                    board.unmake_last();
                }
            }
            let x = cache.entry(board.key()).or_insert((0, true, depth));
            x.2 = x.2.max(depth);
            x.0 = best;
            return best;
        }
        Player::Black => {
            let mut best = isize::MAX;
            for cmove in pseudo_legal_moves.into_iter() {
                if board.make_move(&cmove).is_ok() {
                    best = best.min(eval(board, depth - 1, evaluator, alpha, beta, cache));
                    if best <= alpha {
                        board.unmake_last();
                        break;
                    }
                    beta = beta.min(best);
                    board.unmake_last();
                }
            }
            let x = cache.entry(board.key()).or_insert((0, true, depth));
            x.2 = x.2.max(depth);
            x.0 = best;
            return best;
        }
    }
}

pub struct Bot {
    pub evaluator: Box<dyn Eval>,
    pub search_depth: u8,
    pub cache: FxHashMap<KeyStruct, (isize, bool, u8)>,
}

impl Bot {
    pub fn find_best_move(&mut self, board: &mut Board) -> Option<Move> {
        let mut pseudo_legal_moves = board.get_pseudo_legal_moves();

        let mut rng = rand::thread_rng();
        pseudo_legal_moves.shuffle(&mut rng);

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
                        let val = eval(
                            board,
                            self.search_depth - 1,
                            &self.evaluator,
                            best_move.1,
                            isize::MAX,
                            &mut self.cache,
                        );
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
                            &self.evaluator,
                            best_move.1,
                            isize::MIN,
                            &mut self.cache,
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
