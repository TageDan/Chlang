use std::{borrow::Borrow, collections::HashMap};

use rand::prelude::*;

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
    parent_best: isize,
    cache: &mut HashMap<KeyStruct, (isize, bool)>,
) -> isize {
    match board.get_game_state() {
        GameState::Draw => return 0,
        GameState::Playing => (),
        GameState::Win(Player::White) => return isize::MAX,
        GameState::Win(Player::Black) => return isize::MIN,
    }

    if depth == 0 {
        return evaluator.evaluate(board);
    }

    let mut rng = rand::thread_rng();

    let mut pseudo_legal_moves = board.get_pseudo_legal_moves();

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
            let mut best = isize::MIN;
            for (i, cmove) in pseudo_legal_moves.into_iter().enumerate() {
                if let Some((value, fin)) = cache.get(&board.key()) {
                    if *fin {
                        return *value;
                    } else if *value < best {
                        return *value;
                    }
                }
                if board.make_move(&cmove).is_ok() {
                    if best >= parent_best {
                        board.unmake_last();
                        let x = cache.entry(board.key()).or_insert((0, false));
                        x.0 = best;
                        return best;
                    }
                    best = best.max(eval(board, depth - 1, evaluator, best, cache));
                    board.unmake_last();
                }
            }
            let x = cache.entry(board.key()).or_insert((0, true));
            x.0 = best;
            return best;
        }
        Player::Black => {
            let mut best = isize::MAX;
            for (i, cmove) in pseudo_legal_moves.into_iter().enumerate() {
                if let Some((value, fin)) = cache.get(&board.key()) {
                    if *fin {
                        return *value;
                    } else if *value < best {
                        return *value;
                    }
                }
                if board.make_move(&cmove).is_ok() {
                    if best <= parent_best {
                        board.unmake_last();
                        let x = cache.entry(board.key()).or_insert((0, false));
                        x.0 = best;
                        return best;
                    }
                    best = best.min(eval(board, depth - 1, evaluator, best, cache));
                    board.unmake_last();
                }
            }
            let x = cache.entry(board.key()).or_insert((0, true));
            x.0 = best;
            return best;
        }
    }
}

pub struct Bot {
    pub evaluator: Box<dyn Eval>,
    pub search_depth: u8,
    pub cache: HashMap<KeyStruct, (isize, bool)>,
}

impl Bot {
    pub fn find_best_move(&mut self, board: &mut Board) -> Option<Move> {
        let mut rng = rand::thread_rng();

        let mut pseudo_legal_moves = board.get_pseudo_legal_moves();

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
                            self.evaluator.borrow(),
                            best_move.1,
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
