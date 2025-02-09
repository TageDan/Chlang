use std::{
    io::{BufRead, Read},
    str::FromStr,
};

use base64::Engine;

use crate::tree_evaluator::Eval;

pub struct MaterialEvaluator {
    piece_values: [u8; 6],
}

impl Default for MaterialEvaluator {
    fn default() -> Self {
        Self {
            piece_values: [10, 30, 30, 50, 85, 0],
        }
    }
}

impl From<&[u8]> for MaterialEvaluator {
    fn from(value: &[u8]) -> Self {
        Self {
            piece_values: value[0..6].try_into().unwrap(),
        }
    }
}

impl FromStr for MaterialEvaluator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars()
            .map(|x| match x.is_ascii() {
                true => Ok(x as u8),
                _ => return Err("should be valid ascii"),
            })
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .into())
    }
}

impl From<MaterialEvaluator> for String {
    fn from(value: MaterialEvaluator) -> Self {
        crate::BASE64_STANDARD.encode(value.piece_values)
    }
}

impl Eval for MaterialEvaluator {
    fn evaluate(&self, board: &mut crate::board::Board) -> isize {
        let mut value = 0;
        for piece_index in 0..6 {
            value += self.piece_values[piece_index] as isize
                * (board.white_piece_bitboard & board.piece_bitboards[piece_index]).count_ones()
                    as isize;
            value -= self.piece_values[piece_index] as isize
                * (board.black_piece_bitboard & board.piece_bitboards[piece_index]).count_ones()
                    as isize;
        }
        value
    }
}
