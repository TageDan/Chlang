use crate::board::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn bitboard_index(&self) -> usize {
        match self {
            Self::Pawn => 0,
            Self::Knight => 1,
            Self::Bishop => 2,
            Self::Rook => 3,
            Self::Queen => 4,
            Self::King => 5,
        }
    }

    pub fn index_display_char(index: usize, color: Player) -> char {
        match color {
            Player::White => match index {
                0 => '♙',
                1 => '♘',
                2 => '♗',
                3 => '♖',
                4 => '♕',
                5 => '♔',
                _ => ' ',
            },
            Player::Black => match index {
                0 => '♟',
                1 => '♞',
                2 => '♝',
                3 => '♜',
                4 => '♛',
                5 => '♚',
                _ => ' ',
            },
        }
    }
}
