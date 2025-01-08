use crate::{cmove::Move, piece::Piece};
use std::{error::Error, fmt::Display, str::FromStr};

/// Bitboard representation of a chess board
pub struct Board {
    pub turn: Player,
    pub moves_since_capture: u8,
    pub can_castle_kingside: [bool; 2],
    /// white, black
    pub can_castle_queenside: [bool; 2],
    /// white, black
    pub piece_bitboards: [u64; 6],
    pub white_piece_bitboard: u64,
    pub black_piece_bitboard: u64,
    pub captured_pieces: Vec<(Player, Piece)>,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub col: i64,
    pub row: i64,
}

impl Position {
    pub fn bitboard(&self) -> u64 {
        2_u64.pow((self.row * 8 + self.col) as u32)
    }
    pub fn valid(&self) -> bool {
        self.col < 8 && self.row < 8 && self.col >= 0 && self.row >= 0
    }
    pub fn new<T>(row: T, col: T) -> Self
    where
        T: Into<i64>,
    {
        Position {
            col: col.into(),
            row: row.into(),
        }
    }
}

impl From<u64> for Position {
    fn from(value: u64) -> Self {
        let l = value.ilog2();
        Self::new(l / 8, l % 8)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
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

impl Board {
    pub fn piece_type(&self, pos: &Position) -> Option<(Player, Piece)> {
        let color: Player = if self.white_piece_bitboard & pos.bitboard() != 0 {
            Player::White
        } else if self.black_piece_bitboard & pos.bitboard() != 0 {
            Player::Black
        } else {
            return None;
        };

        for (p_bitboard, p) in (self.piece_bitboards).iter().zip(
            [
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ]
            .into_iter(),
        ) {
            if p_bitboard & pos.bitboard() != 0 {
                return Some((color, p));
            }
        }
        None
    }

    pub fn make_move(&mut self, cmove: Move) -> Result<(), &str> {
        let to = cmove.to().bitboard();
        let from = cmove.from();
        if !self.piece_type(&from).is_some_and(|x| x.0 == self.turn) {
            return Err("Invalid move: Can only move from square occupied by yourself");
        }
        let piece = self.piece_type(&from).ok_or("No piece")?;
        let pseudo_legal_moves = self.get_pseudo_legal_moves_from(&from);
        if !pseudo_legal_moves.contains(&cmove) {
            return Err("Not legal move");
        }
        let from = from.bitboard();
        match self.turn {
            Player::White => {
                if let Some(captured_piece) = self.piece_type(&cmove.to()) {
                    if captured_piece.0 == Player::White {
                        return Err("Invalid move: Cannot capture your own piece");
                    }

                    let cap_bitboard = &mut self.piece_bitboards[captured_piece.1.bitboard_index()];

                    *cap_bitboard = *cap_bitboard & !to;

                    self.black_piece_bitboard = self.black_piece_bitboard & !to;
                    self.captured_pieces.push(captured_piece);
                }
                self.white_piece_bitboard = self.white_piece_bitboard & !from;
                self.white_piece_bitboard = self.white_piece_bitboard | to;
                self.turn = Player::Black;
            }
            Player::Black => {
                if let Some(captured_piece) = self.piece_type(&cmove.to()) {
                    if captured_piece.0 == Player::Black {
                        return Err("Invalid move: Cannot capture your own piece");
                    }

                    let cap_bitboard = &mut self.piece_bitboards[captured_piece.1.bitboard_index()];

                    *cap_bitboard = *cap_bitboard & !to;

                    self.white_piece_bitboard = self.white_piece_bitboard & !to;

                    self.captured_pieces.push(captured_piece);
                }
                self.black_piece_bitboard = self.black_piece_bitboard & !from;
                self.black_piece_bitboard = self.black_piece_bitboard | to;
                self.turn = Player::White;
            }
        }
        let piece_bitboard = &mut self.piece_bitboards[piece.1.bitboard_index()];
        *piece_bitboard = *piece_bitboard & !from;
        *piece_bitboard = *piece_bitboard | to;
        Ok(())
    }

    fn get_pseudo_legal_moves_from(&self, pos: &Position) -> Vec<Move> {
        let moves = match self.piece_type(pos) {
            None => {
                panic!("No piece on square");
            }
            Some((color, piece_type)) => match piece_type {
                Piece::Pawn => self.get_pseudo_legal_pawn_moves_from(pos),
                Piece::Knight => self.get_pseudo_legal_knight_moves_from(pos),
                Piece::Bishop => self.get_pseudo_legal_bishop_moves_from(pos),
                Piece::Rook => self.get_pseudo_legal_rook_moves_from(pos),
                Piece::Queen => {
                    vec![]
                }
                Piece::King => self.get_pseudo_legal_king_moves_from(pos),
            },
        };

        moves
    }

    fn get_pseudo_legal_king_moves_from(&self, pos: &Position) -> Vec<Move> {
        let mut moves = Vec::with_capacity(8);
        for (r, c) in [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ]
        .iter()
        {
            let n_position = Position::new(pos.row + r, pos.col + c);
            if n_position.valid() {
                moves.push(Move::new(pos, &n_position));
            }
        }
        moves
    }

    fn get_pseudo_legal_rook_moves_from(&self, pos: &Position) -> Vec<Move> {
        vec![]
    }

    fn get_pseudo_legal_bishop_moves_from(&self, pos: &Position) -> Vec<Move> {
        vec![]
    }
    fn get_pseudo_legal_knight_moves_from(&self, pos: &Position) -> Vec<Move> {
        let mut moves = Vec::with_capacity(8);
        for (r, c) in [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
        ]
        .iter()
        {
            let n_position = Position::new(pos.row + r, pos.col + c);
            if n_position.valid() {
                moves.push(Move::new(pos, &n_position));
            }
        }
        moves
    }

    fn get_pseudo_legal_pawn_moves_from(&self, pos: &Position) -> Vec<Move> {
        match self.turn {
            Player::White => {
                let mut moves = Vec::with_capacity(4);
                let one_forward = Position::new(pos.row + 1, pos.col);
                let all_bitboard = self.white_piece_bitboard | self.black_piece_bitboard;
                if one_forward.valid() && one_forward.bitboard() & (all_bitboard) == 0 {
                    moves.push(Move::new(pos, &one_forward));
                    if pos.row == 1 {
                        let two_forward = Position::new(3, pos.col);
                        if two_forward.bitboard() & (all_bitboard) == 0 {
                            moves.push(Move::new(pos, &two_forward));
                        }
                    }
                }
                let take_right = Position::new(pos.row + 1, pos.col + 1);
                if take_right.valid() && take_right.bitboard() & self.black_piece_bitboard != 0 {
                    moves.push(Move::new(pos, &take_right));
                }
                let take_left = Position::new(pos.row + 1, pos.col - 1);
                if take_left.valid() && take_left.bitboard() & self.black_piece_bitboard != 0 {
                    moves.push(Move::new(pos, &take_left));
                }
                moves
            }
            Player::Black => {
                let mut moves = Vec::with_capacity(4);
                let one_forward = Position::new(pos.row - 1, pos.col);
                let all_bitboard = self.white_piece_bitboard | self.black_piece_bitboard;
                if one_forward.valid() && one_forward.bitboard() & (all_bitboard) == 0 {
                    moves.push(Move::new(pos, &one_forward));
                    if pos.row == 6 {
                        let two_forward = Position::new(4, pos.col);
                        if two_forward.bitboard() & (all_bitboard) == 0 {
                            moves.push(Move::new(pos, &two_forward));
                        }
                    }
                }
                let take_right = Position::new(pos.row - 1, pos.col + 1);
                if take_right.valid() && take_right.bitboard() & self.white_piece_bitboard != 0 {
                    moves.push(Move::new(pos, &take_right));
                }
                let take_left = Position::new(pos.row - 1, pos.col - 1);
                if take_left.valid() && take_left.bitboard() & (self.white_piece_bitboard) != 0 {
                    moves.push(Move::new(pos, &take_left));
                }

                moves
            }
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
            captured_pieces: vec![],
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
        string.push_str(&format!("turn {:?}", self.turn));
        write!(f, "{string}")
    }
}
