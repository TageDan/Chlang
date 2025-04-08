use std::{
    io::{BufRead, Read},
    ops::Deref,
    str::FromStr,
};

use rand::Rng;

use crate::{
    board::{Player, Position},
    tree_evaluator::Eval,
};

#[derive(Clone, Debug)]
pub struct Evaluator {
    piece_values: [u8; 6],
    piece_positional_values: [[[u8; 8]; 8]; 6],
    piece_attack_values: [u8; 6],
    castle_bonus: [u8; 2],
}

impl Default for Evaluator {
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
            piece_attack_values: [5, 7, 7, 8, 10, 90],
            castle_bonus: [6, 6],
        }
    }
}

impl From<&[u8]> for Evaluator {
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
        let piece_attack_values = value[6 + 8 * 8 * 6..6 + 8 * 8 * 6 + 6].try_into().unwrap();
        let castle_bonus = value[6 + 8 * 8 * 6 + 6..6 + 8 * 8 * 6 + 6 + 2]
            .try_into()
            .unwrap();

        Self {
            piece_values,
            piece_positional_values,
            piece_attack_values,
            castle_bonus,
        }
    }
}

impl FromStr for Evaluator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars()
            .map(|x| match x.is_ascii() {
                true => Ok(x as u8 - 33),
                _ => return Err("should be valid ascii"),
            })
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .into())
    }
}

impl From<Evaluator> for String {
    fn from(value: Evaluator) -> Self {
        let mut bytes = Vec::new();
        value.piece_values.iter().for_each(|x| bytes.push(*x));
        value.piece_positional_values.iter().for_each(|mat| {
            for row in mat {
                for col in row {
                    bytes.push(*col);
                }
            }
        });
        value
            .piece_attack_values
            .iter()
            .for_each(|x| bytes.push(*x));
        value.castle_bonus.iter().for_each(|x| bytes.push(*x));
        bytes.iter_mut().for_each(|x| {
            *x = *x + 33;
            if !(33..=126).contains(x) {
                println!("not in range: {}", *x);
            }
        });
        String::from_utf8(bytes).unwrap()
    }
}

impl Eval for Evaluator {
    fn evaluate(&self, board: &mut crate::board::Board) -> isize {
        let mut value = 0;
        for piece_index in 0..6 {
            let white_pieces = board.white_piece_bitboard & board.piece_bitboards[piece_index];
            let black_pieces = board.black_piece_bitboard & board.piece_bitboards[piece_index];
            value += self.piece_values[piece_index] as isize * (white_pieces.count_ones() as isize);
            value -= self.piece_values[piece_index] as isize * (black_pieces.count_ones() as isize);
            for i in 0..64 {
                if (1 << i) & white_pieces != 0 {
                    let col = i % 8;
                    let row = i / 8;
                    value += self.piece_positional_values[piece_index][row][col] as isize;
                    value -= board
                        .number_of_attacks_by_color(&Position::from(1 << i), &Player::Black)
                        * self.piece_attack_values[piece_index] as isize;
                    value += board
                        .number_of_attacks_by_color(&Position::from(1 << i), &Player::White)
                        * self.piece_attack_values[piece_index] as isize;
                }
                if (1 << i) & black_pieces != 0 {
                    let col = 7 - (i % 8);
                    let row = 7 - i / 8;
                    value -= self.piece_positional_values[piece_index][row][col] as isize;
                    value -= board
                        .number_of_attacks_by_color(&Position::from(1 << i), &Player::Black)
                        * self.piece_attack_values[piece_index] as isize;
                    value += board
                        .number_of_attacks_by_color(&Position::from(1 << i), &Player::White)
                        * self.piece_attack_values[piece_index] as isize;
                }
            }
        }
        if board.can_castle_short[0] {
            value -= self.castle_bonus[0] as isize;
        }
        if board.can_castle_short[1] {
            value += self.castle_bonus[0] as isize;
        }
        if board.can_castle_long[0] {
            value -= self.castle_bonus[1] as isize;
        }
        if board.can_castle_long[1] {
            value += self.castle_bonus[1] as isize;
        }
        value
    }

    fn modified(&self) -> Box<dyn Eval + Sync + Send> {
        let temp = self.clone();
        let string = String::from(temp);
        let bytes = string.as_bytes();
        let mut new_bytes = vec![];
        for b in bytes {
            let nb = b - 33;
            if rand::thread_rng().gen_bool(1.0 / 5.0) {
                let newval = (nb as isize + rand::thread_rng().gen_range(-5..=5));
                let newval = newval.max(0).min(93) as u8;
                new_bytes.push(newval);
            } else {
                new_bytes.push(nb);
            }
        }
        return Box::new(Evaluator::from(new_bytes.as_slice()));
    }
    fn bot_clone(&self) -> Box<dyn Eval + Sync + Send> {
        Box::new(self.clone())
    }
    fn string_rep(&self) -> String {
        String::from(self.clone())
    }
}
