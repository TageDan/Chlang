use std::str::FromStr;

use crate::board::Position;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Move {
    piece: Piece,
    from: Position,
    to: Position,
    capture: bool,
    castle: CastleType,
}

impl FromStr for Move {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "O-O" {
            return Ok(Move {
                piece: Piece::King,
                from: Position { col: 0, row: 0 },
                to: Position { col: 0, row: 0 },
                capture: false,
                castle: CastleType::Short,
            });
        }
        if s == "O-O-O" {
            return Ok(Move {
                piece: Piece::King,
                from: Position { col: 0, row: 0 },
                to: Position { col: 0, row: 0 },
                capture: false,
                castle: CastleType::Long,
            });
        }
        let mut parts = s.chars().peekable();
        if parts.peek().is_none() {
            return Err("empty move string");
        }
        let piece = if parts
            .peek()
            .is_some_and(|x| x.is_ascii_alphabetic() && x.is_uppercase())
        {
            match parts.next().ok_or("no piece")? {
                'R' => Piece::Rook,
                'B' => Piece::Bishop,
                'N' => Piece::Knight,
                'Q' => Piece::Queen,
                'K' => Piece::King,
                _ => return Err("invalid piece error"),
            }
        } else {
            Piece::Pawn
        };
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
                .ok_or("invalid row")?
                - 1;
            if row >= 8 {
                return Err("invalid row error");
            }
            Position { col, row }
        };

        let capture = if *parts.peek().ok_or("no column")? == 'x' {
            parts.next();
            true
        } else {
            false
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
                .ok_or("invalid row")?
                - 1;
            if row >= 8 {
                return Err("invalid row");
            }
            Position { col, row }
        };
        Ok(Move {
            piece,
            from: starting_square,
            to: ending_square,
            capture,
            castle: CastleType::None,
        })
    }
}

#[derive(Debug)]
enum CastleType {
    None,
    Short,
    Long,
}
