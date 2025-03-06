use std::{
    io::{BufRead, Read},
    str::FromStr,
};

use base64::Engine;
use rand::Rng;

use crate::tree_evaluator::Eval;

#[derive(Clone)]
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
        String::from_utf8(value.piece_values.into_iter().collect::<Vec<u8>>()).unwrap()
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
    fn modified(&self) -> Box<dyn Eval> {
        let temp = self.clone();
        let string = String::from(temp);
        let bytes = string.as_bytes();
        let mut new_bytes = vec![];
        for b in bytes {
            if rand::thread_rng().gen_bool(1.0 / 10.0) {
                let newval =
                    ((*b as isize + 128 + rand::thread_rng().gen_range(-1..=1)) % 128) as u8;
                new_bytes.push(newval);
            } else {
                new_bytes.push(*b);
            }
        }
        let new_s = String::from_utf8(new_bytes).unwrap();
        return Box::new(MaterialEvaluator::from_str(&new_s).unwrap());
    }
    fn bot_clone(&self) -> Box<dyn Eval> {
        Box::new(self.clone())
    }
    fn string_rep(&self) -> String {
        String::from(self.clone())
    }
}
