use crate::board::Player;

#[derive(Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
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
