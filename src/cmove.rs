use std::str::FromStr;

use crate::{board::Position, piece::Piece};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    bit_rep: u16,
}

impl FromStr for Move {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().chars().peekable();
        print!("{s}");
        if parts.peek().is_none() {
            return Err("empty move string");
        }
        let starting_square = {
            let col = match parts.next().ok_or("no column")? {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => return Err("invalid column"),
            };
            let row = parts
                .next()
                .ok_or("no row")?
                .to_digit(10)
                .ok_or("invalid row")? as u16
                - 1;
            if row >= 8 {
                return Err("invalid row error");
            }
            col + row * 8
        };

        let ending_square = {
            let col = match parts.next().ok_or("no column")? {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => return Err("invalid column"),
            };
            let row = parts
                .next()
                .ok_or("no row")?
                .to_digit(10)
                .ok_or("invalid row")? as u16
                - 1;
            if row >= 8 {
                return Err("invalid row");
            }
            col + row * 8
        };

        let promotion = match parts.next() {
            Some('R') => Piece::Rook.bitboard_index(),
            Some('B') => Piece::Bishop.bitboard_index(),
            Some('N') => Piece::Knight.bitboard_index(),
            Some('Q') => Piece::Queen.bitboard_index(),
            Some(_) => return Err("Invalid promotion piece"),
            None => 0,
        } as u16;

        Ok(Move {
            bit_rep: starting_square + (ending_square << 6) + (promotion << 12),
        })
    }
}

impl Move {
    pub fn from(&self) -> Position {
        // extract starting square from bits
        let from = (self.bit_rep & 0b111111) as u32;
        let col = from % 8;
        let row = from / 8;
        Position::new(row, col)
    }
    pub fn to(&self) -> Position {
        // extract end square from bits
        let to = (self.bit_rep >> 6 & 0b111111) as u32;
        let col = to % 8;
        let row = to / 8;
        Position::new(row, col)
    }
    pub fn new(from: &Position, to: &Position) -> Self {
        let last_6_bits = (from.row * 8 + from.col) as u16;
        let first_6_bits = ((to.row * 8 + to.col) << 6) as u16;
        Self {
            bit_rep: first_6_bits + last_6_bits,
        }
    }
    pub fn promotion(from: &Position, to: &Position, piece: Piece) -> Self {
        let last_6_bits = (from.row * 8 + from.col) as u16;
        let first_6_bits = ((to.row * 8 + to.col) << 6) as u16;
        let promotion_bits = (piece.bitboard_index() << 12) as u16;
        Self {
            bit_rep: promotion_bits + first_6_bits + last_6_bits,
        }
        // AAAAH: I really love this move representation. Soooo clean. Wish I had the time to figure out how to make the board this clean aswell. (I guess the board could really be represented as an initial state and a list of made moves. But that would probably be really slow. It would be like my unmake move function).
    }
    pub fn promotion_bitboard_index(&self) -> usize {
        (self.bit_rep >> 12) as usize
    }

    pub fn without_promotion(&self) -> Move {
        Move {
            bit_rep: self.bit_rep & 0b111111111111,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CastleType {
    None,
    Short,
    Long,
}
