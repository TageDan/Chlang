use std::{
    io::{BufRead, Read},
    str::FromStr,
};

use base64::Engine;

use crate::tree_evaluator::Eval;

pub struct PositionalEvaluator {
    piece_values: [u8; 6],
    piece_positional_values: [[[u8; 8]; 8]; 6],
}

impl Default for PositionalEvaluator {
    fn default() -> Self {
        Self {
            piece_values: [10, 30, 30, 50, 85, 0],
            piece_positional_values: [
                [
                    // pawn
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0],
                    [1, 1, 1, 2, 2, 1, 1, 1],
                    [2, 2, 2, 2, 2, 2, 2, 2],
                    [3, 3, 3, 3, 3, 3, 3, 3],
                    [4, 4, 4, 4, 4, 4, 4, 4],
                    [5, 5, 5, 5, 5, 5, 5, 5],
                ],
                [
                    // knight
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0],
                    [0, 1, 2, 2, 2, 2, 1, 0],
                    [0, 1, 2, 3, 3, 2, 1, 0],
                    [0, 1, 2, 3, 3, 2, 1, 0],
                    [0, 1, 2, 2, 2, 2, 1, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0],
                ],
                [
                    // bishop
                    [2, 0, 0, 0, 0, 0, 0, 2],
                    [0, 3, 1, 0, 0, 1, 3, 0],
                    [0, 1, 3, 2, 2, 3, 1, 0],
                    [0, 1, 2, 3, 3, 2, 1, 0],
                    [0, 1, 2, 3, 3, 2, 1, 0],
                    [0, 1, 3, 2, 2, 3, 1, 0],
                    [0, 3, 1, 1, 1, 1, 3, 0],
                    [2, 0, 0, 0, 0, 0, 0, 2],
                ],
                [
                    // rook
                    [0, 1, 2, 2, 2, 2, 1, 0],
                    [0, 1, 3, 3, 3, 3, 1, 0],
                    [0, 1, 2, 2, 2, 2, 1, 0],
                    [0, 1, 2, 2, 2, 2, 1, 0],
                    [0, 1, 2, 2, 2, 2, 1, 0],
                    [0, 2, 3, 3, 3, 3, 2, 0],
                    [5, 5, 5, 5, 5, 5, 5, 5],
                    [2, 2, 2, 2, 2, 2, 2, 2],
                ],
                [
                    // queen
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [1, 1, 2, 1, 1, 2, 1, 1],
                    [1, 1, 1, 0, 0, 1, 1, 1],
                    [1, 1, 1, 0, 0, 1, 1, 1],
                    [1, 1, 2, 1, 1, 2, 1, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                ],
                [
                    // king
                    [3, 2, 1, 0, 0, 1, 2, 3],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                ],
            ],
        }
    }
}

impl From<&[u8]> for PositionalEvaluator {
    fn from(value: &[u8]) -> Self {
        let piece_values = value[0..6].try_into().unwrap();
        let mut piece_positional_values = [[[0; 8]; 8]; 6];
        let mut current_type: isize = -1;
        for (i, b) in value[6..6 + 8 * 8 * 6].iter().enumerate() {
            if i % (8 * 8) == 0 {
                current_type += 1;
            }
            let j = i - current_type as usize * 8 * 8;
            piece_positional_values[current_type as usize][j / 8][j % 8] = *b;
        }

        Self {
            piece_values,
            piece_positional_values,
        }
    }
}

impl FromStr for PositionalEvaluator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.as_bytes().into())
    }
}

impl From<PositionalEvaluator> for String {
    fn from(value: PositionalEvaluator) -> Self {
        let mut bytes = Vec::new();
        value.piece_values.iter().for_each(|x| bytes.push(*x));
        value.piece_positional_values.iter().for_each(|mat| {
            for row in mat {
                for col in row {
                    bytes.push(*col);
                }
            }
        });
        crate::BASE64_STANDARD.encode(bytes)
    }
}

impl Eval for PositionalEvaluator {
    fn evaluate(&self, board: &mut crate::board::Board) -> isize {
        let mut value = 0;
        for piece_index in 0..6 {
            let white_pieces = board.white_piece_bitboard & board.piece_bitboards[piece_index];
            let black_pieces = board.black_piece_bitboard & board.piece_bitboards[piece_index];
            value += self.piece_values[piece_index] as isize * (white_pieces).count_ones() as isize;
            value -= self.piece_values[piece_index] as isize * (black_pieces).count_ones() as isize;
            for i in 0..64 {
                if 1 << i & white_pieces != 0 {
                    let col = i % 8;
                    let row = i / 8;
                    value += self.piece_positional_values[piece_index][row][col] as isize;
                }
                if 1 << i & black_pieces != 0 {
                    let col = i % 8;
                    let row = 7 - i / 8;
                    value -= self.piece_positional_values[piece_index][row][col] as isize;
                }
            }
        }
        value
    }
}
