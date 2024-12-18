use std::{fmt::Display, str::FromStr};

/// Bitboard representation of a chess board
pub struct Board {
    pub turn: Player,
    pub moves_since_capture: u8,
    pub can_castle_kingside: [bool; 2],
    pub can_castle_queenside: [bool; 2],
    pub piece_bitboards: [u64; 6],
    pub white_piece_bitboard: u64,
    pub black_piece_bitboard: u64,
}

struct Position {
    col: u32,
    row: u32,
}

struct Move {
    piece: Piece,
    from: Position,
    to: Position,
    capture: bool,
    castle: CastleType,
}

enum CastleType {
    None,
    Short,
    Long,
}

enum Player {
    White,
    Black,
}

impl Player {
    fn idx(&self) -> usize {
        match self {
            Self::White => 1,
            Self::Black => 0,
        }
    }
}

enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    fn index_display_char(index: usize, color: Player) -> char {
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
                4 => '♜',
                5 => '♚',
                _ => ' ',
            },
        }
    }
}

impl Default for Board {
    /// Return the initial position
    fn default() -> Self {
        Self {
            turn: Player::White,
            moves_since_capture: 0,
            can_castle_kingside: [true, true],
            can_castle_queenside: [true, true],
            piece_bitboards: [
                0xff00000000ff00,   // pawns
                0x4200000000000042, // knights
                0x2400000000000024, // bishops
                0x8100000000000081, // rook
                0x800000000000008,  // king
                0x1000000000000010, // queen
            ],
            white_piece_bitboard: 0xffff,
            black_piece_bitboard: 0xffff000000000000,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let on = "\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m";
        let off = "\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m";
        let mut string = String::from_str(on).unwrap();
        string.push_str("+---+---+---+---+---+---+---+---+");
        string.push_str(off);
        string.push('\n');
        for y in 0..8 {
            string.push_str(on);
            for x in 0..8 {
                let mut c = ' ';
                for (i, bit_board) in self.piece_bitboards.iter().enumerate() {
                    if bit_board & self.white_piece_bitboard & 2_u64.pow(y * 8 + x) != 0 {
                        c = Piece::index_display_char(i, Player::White)
                    }
                    if bit_board & self.black_piece_bitboard & 2_u64.pow(y * 8 + x) != 0 {
                        c = Piece::index_display_char(i, Player::Black)
                    }
                }
                string.push_str(&format!("| {c} "));
            }
            string.push('|');
            string.push_str(off);
            string.push('\n');
            string.push_str(on);
            string.push_str("                                 ");
            string.push_str(off);
            string.push('\n');
            string.push_str(on);
            string.push_str("+---+---+---+---+---+---+---+---+");
            string.push_str(off);
            string.push('\n');
            string.push_str(on);
            string.push_str("                                 ");
            string.push_str(off);
            string.push('\n');
        }
        write!(f, "{string}")
    }
}
