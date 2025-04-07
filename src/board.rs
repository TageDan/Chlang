use rustc_hash::FxHashMap;

use crate::{cmove::Move, piece::Piece};

use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Stdin},
    str::FromStr,
};

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
    Playing,
    Win(Player),
    Draw,
}

/// Bitboard representation of a chess board
#[derive(PartialEq, Clone)]
pub struct Board {
    pub turn: Player,
    pub moves_since_capture: u8,
    /// black, white
    pub can_castle_short: [bool; 2],
    /// black, white
    pub can_castle_long: [bool; 2],
    pub piece_bitboards: [u64; 6],
    pub white_piece_bitboard: u64,
    pub black_piece_bitboard: u64,
    pub possible_en_passant: Option<Position>,
    pub previous_board_states: Vec<(KeyStruct, u8)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub col: i64,
    pub row: i64,
}

impl Position {
    pub fn bitboard(&self) -> u64 {
        1 << (self.row * 8 + self.col) as u32
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
        if value == 0 {
            return Self::new(0, 0);
        }
        let l = value.ilog2();
        Self::new(l / 8, l % 8)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn idx(&self) -> usize {
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

    // Make a move or return an error if move is not valid
    pub fn make_move(&mut self, cmove: &Move) -> Result<(), &str> {
        let mut to = cmove.to().bitboard();
        let from = cmove.from();
        if !self.piece_type(&from).is_some_and(|x| x.0 == self.turn) {
            return Err("Invalid move: Can only move from square occupied by yourself");
        }
        let piece = self.piece_type(&from).ok_or("No piece")?;
        let pseudo_legal_moves = self.get_pseudo_legal_moves_from_pos(&from);
        if !pseudo_legal_moves.contains(&cmove) {
            return Err("Not legal move");
        }
        let from = from.bitboard();
        let mut new_long_castle_rights = true;
        let mut new_short_castle_rights = true;
        let mut capture = false;
        let old_board_state = self.key();
        match self.turn {
            Player::White => {
                new_short_castle_rights = self.can_castle_short[Player::White.idx()];
                new_long_castle_rights = self.can_castle_long[Player::White.idx()];

                // Special pawn handling
                if piece.1 == Piece::Pawn {
                    // Must promote if going to last rank
                    if cmove.to().row == 7 {
                        if cmove.promotion_bitboard_index() == 0 {
                            return Err("No promotion piece on promotion move");
                        }
                    }
                    // En passant handling
                    if let Some(ref p) = self.possible_en_passant {
                        if *p == cmove.to() {
                            let mut piece_pos = cmove.to();
                            piece_pos.row -= 1;
                            let cap_bitboard =
                                &mut self.piece_bitboards[Piece::Pawn.bitboard_index()];
                            *cap_bitboard = *cap_bitboard & !piece_pos.bitboard();

                            self.black_piece_bitboard =
                                self.black_piece_bitboard & !piece_pos.bitboard();
                            capture = true;
                        }
                    }
                }

                // Captures
                if let Some(captured_piece) = self.piece_type(&cmove.to()) {
                    if captured_piece.0 == Player::White {
                        return Err("Invalid move: Cannot capture your own piece");
                    }

                    let cap_bitboard = &mut self.piece_bitboards[captured_piece.1.bitboard_index()];

                    *cap_bitboard = *cap_bitboard & !to;

                    self.black_piece_bitboard = self.black_piece_bitboard & !to;
                    capture = true;
                }

                // Castle handling
                if piece.1 == Piece::King && (cmove.to().col - cmove.from().col).abs() >= 2 {
                    match cmove.to().col {
                        2 => {
                            // long castle

                            let rook_bitboard =
                                &mut self.piece_bitboards[Piece::Rook.bitboard_index()];

                            // Remove old rook
                            self.white_piece_bitboard =
                                self.white_piece_bitboard & !Position::new(0, 0).bitboard();

                            *rook_bitboard = *rook_bitboard & !Position::new(0, 0).bitboard();

                            // Add new rook
                            self.white_piece_bitboard =
                                self.white_piece_bitboard | Position::new(0, 3).bitboard();

                            *rook_bitboard = *rook_bitboard | Position::new(0, 3).bitboard();
                        }
                        6 => {
                            // short castle

                            let rook_bitboard =
                                &mut self.piece_bitboards[Piece::Rook.bitboard_index()];

                            // Remove old rook
                            self.white_piece_bitboard =
                                self.white_piece_bitboard & !Position::new(0, 7).bitboard();

                            *rook_bitboard = *rook_bitboard & !Position::new(0, 7).bitboard();

                            // Add new rook
                            self.white_piece_bitboard =
                                self.white_piece_bitboard | Position::new(0, 5).bitboard();

                            *rook_bitboard = *rook_bitboard | Position::new(0, 5).bitboard();
                        }
                        _ => return Err("invalid_move"),
                    }
                }

                self.white_piece_bitboard = self.white_piece_bitboard & !from;
                self.white_piece_bitboard = self.white_piece_bitboard | to;
                self.turn = Player::Black;
            }
            Player::Black => {
                new_short_castle_rights = self.can_castle_short[Player::Black.idx()];
                new_long_castle_rights = self.can_castle_long[Player::Black.idx()];
                // Special pawn handling
                if piece.1 == Piece::Pawn {
                    // Must promote if going to last rank
                    if cmove.to().row == 0 {
                        if cmove.promotion_bitboard_index() == 0 {
                            return Err("No promotion piece on promotion move");
                        }
                    }
                    // En passant handling
                    if let Some(ref p) = self.possible_en_passant {
                        if *p == cmove.to() {
                            let mut piece_pos = cmove.to();
                            piece_pos.row += 1;
                            let cap_bitboard =
                                &mut self.piece_bitboards[Piece::Pawn.bitboard_index()];
                            *cap_bitboard = *cap_bitboard & !piece_pos.bitboard();
                            self.white_piece_bitboard =
                                self.white_piece_bitboard & !piece_pos.bitboard();
                            capture = true;
                        }
                    }
                }

                // Captures
                if let Some(captured_piece) = self.piece_type(&cmove.to()) {
                    if captured_piece.0 == Player::Black {
                        return Err("Invalid move: Cannot capture your own piece");
                    }

                    let cap_bitboard = &mut self.piece_bitboards[captured_piece.1.bitboard_index()];

                    *cap_bitboard = *cap_bitboard & !to;

                    self.white_piece_bitboard = self.white_piece_bitboard & !to;
                    capture = true;
                }

                // Castle handling
                if piece.1 == Piece::King && (cmove.to().col - cmove.from().col).abs() >= 2 {
                    match cmove.to().col {
                        2 => {
                            // long castle

                            let rook_bitboard =
                                &mut self.piece_bitboards[Piece::Rook.bitboard_index()];

                            // Remove old rook
                            self.black_piece_bitboard =
                                self.black_piece_bitboard & !Position::new(7, 0).bitboard();

                            *rook_bitboard = *rook_bitboard & !Position::new(7, 0).bitboard();

                            // Add new rook
                            self.black_piece_bitboard =
                                self.black_piece_bitboard | Position::new(7, 3).bitboard();

                            *rook_bitboard = *rook_bitboard | Position::new(7, 3).bitboard();
                        }
                        6 => {
                            // short castle

                            let rook_bitboard =
                                &mut self.piece_bitboards[Piece::Rook.bitboard_index()];

                            // Remove old rook
                            self.black_piece_bitboard =
                                self.black_piece_bitboard & !Position::new(7, 7).bitboard();

                            *rook_bitboard = *rook_bitboard & !Position::new(7, 7).bitboard();

                            // Add new rook
                            self.black_piece_bitboard =
                                self.black_piece_bitboard | Position::new(7, 5).bitboard();

                            *rook_bitboard = *rook_bitboard | Position::new(7, 5).bitboard();
                        }
                        _ => return Err("invalid_move"),
                    }
                }

                self.black_piece_bitboard = self.black_piece_bitboard & !from;
                self.black_piece_bitboard = self.black_piece_bitboard | to;
                self.turn = Player::White;
            }
        }

        // Promotion
        if piece.1 == Piece::Pawn && (cmove.to().row == 0 || cmove.to().row == 7) {
            let bitboard_index = cmove.promotion_bitboard_index();
            let piece_bitboard = &mut self.piece_bitboards[bitboard_index];

            *piece_bitboard = *piece_bitboard | to;
            let pawn_bitboard = &mut self.piece_bitboards[Piece::Pawn.bitboard_index()];
            *pawn_bitboard = *pawn_bitboard & !from
        }
        // No promotion
        else {
            let piece_bitboard = &mut self.piece_bitboards[piece.1.bitboard_index()];

            *piece_bitboard = *piece_bitboard & !from;
            *piece_bitboard = *piece_bitboard | to;
        }

        self.previous_board_states
            .push((old_board_state, self.moves_since_capture));

        if !self.is_valid() {
            self.unmake_last();
            return Err("This leaves the king in check");
        }

        // Update en passant rules
        if piece.1 == Piece::King {
            new_short_castle_rights = false;
            new_long_castle_rights = false;
        }

        // Update en passant rules
        if piece.1 == Piece::Rook {
            match cmove.from().col {
                0 => {
                    new_long_castle_rights = false;
                }
                7 => {
                    new_short_castle_rights = false;
                }
                _ => (),
            }
        }

        // If moved pawn two steps. Set possible en passant to en passant location.
        self.possible_en_passant = if piece.1 == Piece::Pawn {
            if (cmove.to().row - cmove.from().row).abs() == 2 {
                Some(Position::new(
                    (cmove.to().row + cmove.from().row) / 2,
                    cmove.from().col,
                ))
            } else {
                None
            }
        } else {
            None
        };
        match self.turn {
            Player::Black => {
                self.can_castle_short[Player::White.idx()] = new_short_castle_rights;
                self.can_castle_long[Player::White.idx()] = new_long_castle_rights;
            }
            Player::White => {
                self.can_castle_short[Player::Black.idx()] = new_short_castle_rights;
                self.can_castle_long[Player::Black.idx()] = new_long_castle_rights;
            }
        }
        if capture {
            self.moves_since_capture = 0;
        } else {
            self.moves_since_capture += 1;
        }
        Ok(())
    }

    /// Validate that king isn't in check on start of opponents turn
    fn is_valid(&mut self) -> bool {
        match self.turn {
            Player::White => {
                let king_pos = Position::from(
                    self.piece_bitboards[Piece::King.bitboard_index()] & self.black_piece_bitboard,
                );

                return !self.attacked_by_color(&king_pos, &Player::White);
            }
            Player::Black => {
                let king_pos = Position::from(
                    self.piece_bitboards[Piece::King.bitboard_index()] & self.white_piece_bitboard,
                );

                return !self.attacked_by_color(&king_pos, &Player::Black);
            }
        }
    }

    pub fn get_pseudo_legal_moves_from_pos(&self, pos: &Position) -> Vec<Move> {
        let moves = match self.piece_type(pos) {
            None => {
                panic!("No piece on square");
            }
            Some((color, piece_type)) => match piece_type {
                Piece::Pawn => self.get_pseudo_legal_pawn_moves_from_pos(pos, &color),
                Piece::Knight => self.get_pseudo_legal_knight_moves_from_pos(pos, &color),
                Piece::Bishop => self.get_pseudo_legal_bishop_moves_from_pos(pos, &color),
                Piece::Rook => self.get_pseudo_legal_rook_moves_from_pos(pos, &color),
                Piece::Queen => [
                    self.get_pseudo_legal_rook_moves_from_pos(pos, &color),
                    self.get_pseudo_legal_bishop_moves_from_pos(pos, &color),
                ]
                .concat(),
                Piece::King => self.get_pseudo_legal_king_moves_from_pos(pos, &color, true),
            },
        };

        moves
    }

    fn get_pseudo_legal_king_moves_from_pos(
        &self,
        pos: &Position,
        color: &Player,
        with_castles: bool,
    ) -> Vec<Move> {
        let all_piece_bitboard = self.white_piece_bitboard | self.black_piece_bitboard;
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
        match self.turn {
            Player::White => {
                // check if pieces are blocking between king and rook
                if with_castles
                    && self.can_castle_short[Player::White.idx()]
                    && (self.white_piece_bitboard | self.black_piece_bitboard) & 0x60 == 0
                {
                    if !(self.attacked_by_color(pos, &Player::Black)
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col + 1),
                            &Player::Black,
                        )
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col + 2),
                            &Player::Black,
                        ))
                    {
                        moves.push(Move::new(
                            pos,
                            &Position {
                                col: pos.col + 2,
                                row: pos.row,
                            },
                        ))
                    }
                }
                // check if pieces are blocking between king and rook
                if with_castles
                    && self.can_castle_long[Player::White.idx()]
                    && (self.white_piece_bitboard | self.black_piece_bitboard) & 0xe == 0
                {
                    // check if intermediate positions are attacked
                    if !(self.attacked_by_color(pos, &Player::Black)
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col - 1),
                            &Player::Black,
                        )
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col - 2),
                            &Player::Black,
                        )
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col - 3),
                            &Player::Black,
                        ))
                    {
                        moves.push(Move::new(
                            pos,
                            &Position {
                                col: pos.col - 2,
                                row: pos.row,
                            },
                        ))
                    }
                }
            }

            Player::Black => {
                // check if pieces are blocking between king and rook
                if with_castles
                    && self.can_castle_short[Player::Black.idx()]
                    && (self.white_piece_bitboard | self.black_piece_bitboard) & 0x6000000000000000
                        == 0
                {
                    // check if intermediate positions are attacked
                    if !(self.attacked_by_color(pos, &Player::White)
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col + 1),
                            &Player::White,
                        )
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col + 2),
                            &Player::White,
                        ))
                    {
                        moves.push(Move::new(
                            pos,
                            &Position {
                                col: pos.col + 2,
                                row: pos.row,
                            },
                        ))
                    }
                }
                // check if pieces are blocking between king and rook
                if with_castles
                    && self.can_castle_long[Player::Black.idx()]
                    && (self.white_piece_bitboard | self.black_piece_bitboard) & 0xe00000000000000
                        == 0
                {
                    // check if intermediate positions are attacked
                    if !(self.attacked_by_color(pos, &Player::White)
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col - 1),
                            &Player::White,
                        )
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col - 2),
                            &Player::White,
                        )
                        || self.attacked_by_color(
                            &Position::new(pos.row, pos.col - 3),
                            &Player::White,
                        ))
                    {
                        moves.push(Move::new(
                            pos,
                            &Position {
                                col: pos.col - 2,
                                row: pos.row,
                            },
                        ))
                    }
                }
            }
        }
        moves
    }

    pub fn get_pseudo_legal_rook_moves_from_pos(
        &self,
        pos: &Position,
        color: &Player,
    ) -> Vec<Move> {
        let mut moves = Vec::with_capacity(16);
        let all_bitboard = self.white_piece_bitboard | self.black_piece_bitboard;
        let opponent_bitboard = match color {
            Player::White => self.black_piece_bitboard,
            Player::Black => self.white_piece_bitboard,
        };
        for (r, c) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let mut n_pos = Position::new(pos.row + r, pos.col + c);
            while n_pos.valid() && n_pos.bitboard() & all_bitboard == 0 {
                moves.push(Move::new(pos, &n_pos));
                n_pos.col += c;
                n_pos.row += r;
            }
            if n_pos.valid() && n_pos.bitboard() & opponent_bitboard != 0 {
                moves.push(Move::new(pos, &n_pos));
            }
        }
        moves
    }

    pub fn get_pseudo_legal_bishop_moves_from_pos(
        &self,
        pos: &Position,
        color: &Player,
    ) -> Vec<Move> {
        let mut moves = Vec::with_capacity(16);
        let all_bitboard = self.white_piece_bitboard | self.black_piece_bitboard;
        let opponent_bitboard = match color {
            Player::White => self.black_piece_bitboard,
            Player::Black => self.white_piece_bitboard,
        };
        for (r, c) in [(1, 1), (-1, 1), (1, -1), (-1, -1)].iter() {
            let mut n_pos = Position::new(pos.row + r, pos.col + c);
            while n_pos.valid() && n_pos.bitboard() & all_bitboard == 0 {
                moves.push(Move::new(pos, &n_pos));
                n_pos.col += c;
                n_pos.row += r;
            }
            if n_pos.valid() && n_pos.bitboard() & opponent_bitboard != 0 {
                moves.push(Move::new(pos, &n_pos));
            }
        }
        moves
    }
    pub fn get_pseudo_legal_knight_moves_from_pos(
        &self,
        pos: &Position,
        color: &Player,
    ) -> Vec<Move> {
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

    pub fn get_pseudo_legal_pawn_moves_from_pos(
        &self,
        pos: &Position,
        color: &Player,
    ) -> Vec<Move> {
        match color {
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
                if take_right.valid()
                    && (take_right.bitboard() & self.black_piece_bitboard != 0
                        || self
                            .possible_en_passant
                            .as_ref()
                            .is_some_and(|x| *x == take_right))
                {
                    moves.push(Move::new(pos, &take_right));
                }
                let take_left = Position::new(pos.row + 1, pos.col - 1);
                if take_left.valid()
                    && (take_left.bitboard() & self.black_piece_bitboard != 0
                        || self
                            .possible_en_passant
                            .as_ref()
                            .is_some_and(|x| *x == take_left))
                {
                    moves.push(Move::new(pos, &take_left));
                }
                for m in moves.iter_mut() {
                    let to = m.to();
                    if to.row == 7 {
                        let from = m.from();
                        for piece in [Piece::Knight, Piece::Rook, Piece::Bishop, Piece::Queen] {
                            *m = Move::promotion(&from, &m.to(), piece);
                        }
                    }
                }
                return moves;
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
                if take_right.valid()
                    && (take_right.bitboard() & self.white_piece_bitboard != 0
                        || self
                            .possible_en_passant
                            .as_ref()
                            .is_some_and(|x| *x == take_right))
                {
                    moves.push(Move::new(pos, &take_right));
                }
                let take_left = Position::new(pos.row - 1, pos.col - 1);
                if take_left.valid()
                    && (take_left.bitboard() & (self.white_piece_bitboard) != 0
                        || self
                            .possible_en_passant
                            .as_ref()
                            .is_some_and(|x| *x == take_left))
                {
                    moves.push(Move::new(pos, &take_left));
                }

                for m in moves.iter_mut() {
                    let to = m.to();
                    if to.row == 0 {
                        let from = m.from();
                        for piece in [Piece::Knight, Piece::Rook, Piece::Bishop, Piece::Queen] {
                            *m = Move::promotion(&from, &m.to(), piece);
                        }
                    }
                }
                return moves;
            }
        }
    }

    /// Returns the game state of this [`Board`].
    /// doesn't actually mutate the inner value. Just does it in
    /// intermiediate steps but then undo's thoose operations.
    /// I know, I know, Bad practice, Maybe I'll fix it later.
    pub fn get_game_state(&mut self) -> GameState {
        if self.is_fifty_move_rule() || self.is_threefold_rep() {
            return GameState::Draw;
        }
        match self.turn {
            Player::White => {
                if self.get_valid_moves().is_empty() {
                    if self.attacked_by_color(
                        &Position::from(
                            self.piece_bitboards[Piece::King.bitboard_index()]
                                & self.white_piece_bitboard,
                        ),
                        &Player::Black,
                    ) {
                        return GameState::Win(Player::Black);
                    };
                    return GameState::Draw;
                }
            }

            Player::Black => {
                if self.get_valid_moves().is_empty() {
                    if self.attacked_by_color(
                        &Position::from(
                            self.piece_bitboards[Piece::King.bitboard_index()]
                                & self.black_piece_bitboard,
                        ),
                        &Player::White,
                    ) {
                        return GameState::Win(Player::White);
                    }
                    return GameState::Draw;
                }
            }
        }
        GameState::Playing
    }

    /// unmake the last move on the board
    pub fn unmake_last(&mut self) {
        let (
            KeyStruct {
                turn,
                piece_bitboards,
                white_piece_bitboard,
                black_piece_bitboard,
                castle_short,
                castle_long,
                possible_en_passant,
            },
            moves_since_capture,
        ) = self
            .previous_board_states
            .pop()
            .expect("Tried to undo initial state");
        self.turn = turn;
        self.piece_bitboards = piece_bitboards;
        self.white_piece_bitboard = white_piece_bitboard;
        self.black_piece_bitboard = black_piece_bitboard;
        self.can_castle_short = castle_short;
        self.can_castle_long = castle_long;
        self.possible_en_passant = possible_en_passant;
        self.moves_since_capture = moves_since_capture;
    }

    pub fn number_of_attacks_by_color(&self, pos: &Position, color: &Player) -> isize {
        let mut num = 0;
        match color {
            Player::Black => {
                for cmove in self.get_pseudo_legal_king_moves_from_pos(&pos, &Player::White, false)
                {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::Black && x.1 == Piece::King)
                    {
                        // attacked by king
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_bishop_moves_from_pos(&pos, &Player::White) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::Black && (x.1 == Piece::Bishop || x.1 == Piece::Queen)
                    }) {
                        // attacked by bishop or queen
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_rook_moves_from_pos(&pos, &Player::White) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::Black && (x.1 == Piece::Rook || x.1 == Piece::Queen)
                    }) {
                        // attacked by Rook or queen
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_knight_moves_from_pos(&pos, &Player::White) {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::Black && x.1 == Piece::Knight)
                    {
                        // attacked by knight
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_pawn_moves_from_pos(&pos, &Player::White) {
                    if cmove.to().col == pos.col {
                        continue;
                    }
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::Black && x.1 == Piece::Pawn)
                    {
                        // attacked by pawn
                        num += 1;
                    }
                }
                return num;
            }
            Player::White => {
                for cmove in self.get_pseudo_legal_king_moves_from_pos(&pos, &Player::Black, false)
                {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::White && x.1 == Piece::King)
                    {
                        // attacked by king
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_bishop_moves_from_pos(&pos, &Player::Black) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::White && (x.1 == Piece::Bishop || x.1 == Piece::Queen)
                    }) {
                        // attacked by bishop or queen
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_rook_moves_from_pos(&pos, &Player::Black) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::White && (x.1 == Piece::Rook || x.1 == Piece::Queen)
                    }) {
                        // attacked by Rook or queen
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_knight_moves_from_pos(&pos, &Player::Black) {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::White && x.1 == Piece::Knight)
                    {
                        // attacked by knight
                        num += 1;
                    }
                }
                for cmove in self.get_pseudo_legal_pawn_moves_from_pos(&pos, &Player::Black) {
                    if cmove.to().col == pos.col {
                        continue;
                    }
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::White && x.1 == Piece::Pawn)
                    {
                        // attacked by pawn
                        num += 1;
                    }
                }
                return num;
            }
        }
    }

    pub fn attacked_by_color(&self, pos: &Position, color: &Player) -> bool {
        match color {
            Player::Black => {
                for cmove in self.get_pseudo_legal_king_moves_from_pos(&pos, &Player::White, false)
                {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::Black && x.1 == Piece::King)
                    {
                        // attacked by king
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_bishop_moves_from_pos(&pos, &Player::White) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::Black && (x.1 == Piece::Bishop || x.1 == Piece::Queen)
                    }) {
                        // attacked by bishop or queen
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_rook_moves_from_pos(&pos, &Player::White) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::Black && (x.1 == Piece::Rook || x.1 == Piece::Queen)
                    }) {
                        // attacked by Rook or queen
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_knight_moves_from_pos(&pos, &Player::White) {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::Black && x.1 == Piece::Knight)
                    {
                        // attacked by knight
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_pawn_moves_from_pos(&pos, &Player::White) {
                    if cmove.to().col == pos.col {
                        continue;
                    }
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::Black && x.1 == Piece::Pawn)
                    {
                        // attacked by pawn
                        return true;
                    }
                }
                return false;
            }
            Player::White => {
                for cmove in self.get_pseudo_legal_king_moves_from_pos(&pos, &Player::Black, false)
                {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::White && x.1 == Piece::King)
                    {
                        // attacked by king
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_bishop_moves_from_pos(&pos, &Player::Black) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::White && (x.1 == Piece::Bishop || x.1 == Piece::Queen)
                    }) {
                        // attacked by bishop or queen
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_rook_moves_from_pos(&pos, &Player::Black) {
                    if self.piece_type(&cmove.to()).is_some_and(|x| {
                        x.0 == Player::White && (x.1 == Piece::Rook || x.1 == Piece::Queen)
                    }) {
                        // attacked by Rook or queen
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_knight_moves_from_pos(&pos, &Player::Black) {
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::White && x.1 == Piece::Knight)
                    {
                        // attacked by knight
                        return true;
                    }
                }
                for cmove in self.get_pseudo_legal_pawn_moves_from_pos(&pos, &Player::Black) {
                    if cmove.to().col == pos.col {
                        continue;
                    }
                    if self
                        .piece_type(&cmove.to())
                        .is_some_and(|x| x.0 == Player::White && x.1 == Piece::Pawn)
                    {
                        // attacked by pawn
                        return true;
                    }
                }
                return false;
            }
        }
    }

    fn is_fifty_move_rule(&self) -> bool {
        if self.moves_since_capture >= 100 {
            return true;
        }
        return false;
    }
    pub fn key(&self) -> KeyStruct {
        KeyStruct {
            turn: self.turn.clone(),
            piece_bitboards: self.piece_bitboards,
            white_piece_bitboard: self.white_piece_bitboard,
            black_piece_bitboard: self.black_piece_bitboard,
            castle_short: self.can_castle_short,
            castle_long: self.can_castle_long,
            possible_en_passant: self.possible_en_passant.clone(),
        }
    }

    fn is_threefold_rep(&self) -> bool {
        let mut counts = FxHashMap::default();
        let mut piece_count = (self.white_piece_bitboard | self.black_piece_bitboard).count_ones();
        for (x, _) in self.previous_board_states.iter().rev() {
            if (x.white_piece_bitboard | x.black_piece_bitboard).count_ones() != piece_count {
                piece_count = (x.white_piece_bitboard | x.black_piece_bitboard).count_ones();
                return false;
            } else {
                let count = counts.entry(x).or_insert(0);
                *count += 1;
                if *count == 3 {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_valid_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(40 * 56);
        for cmove in self.get_pseudo_legal_moves() {
            if self.make_move(&cmove).is_ok() {
                self.unmake_last();
                moves.push(cmove);
            }
        }
        moves
    }

    pub fn get_pseudo_legal_moves(&self) -> Vec<Move> {
        let turn_bitboard = match self.turn {
            Player::White => self.white_piece_bitboard,
            Player::Black => self.black_piece_bitboard,
        };
        // There can only be 40 pieces maximum
        let mut positions = Vec::with_capacity(40);
        for i in 0..64_u32 {
            let pos_bitboard = 2_u64.pow(i);
            if pos_bitboard & turn_bitboard != 0 {
                positions.push(Position::from(pos_bitboard & turn_bitboard));
            }
        }
        // Each piece can at most go to 56 squares (queens)
        let mut moves = Vec::with_capacity(positions.len() * 56);
        for pos in &positions {
            for cmove in self.get_pseudo_legal_moves_from_pos(pos) {
                moves.push(cmove);
            }
        }
        moves
    }
}

impl Default for Board {
    /// Return the initial position
    fn default() -> Self {
        Self {
            turn: Player::White,
            moves_since_capture: 0,
            can_castle_long: [true, true],
            can_castle_short: [true, true],
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
            possible_en_passant: None,
            previous_board_states: Vec::with_capacity(300),
        }
    }
}

// Struct used as a key_representing a board_state
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct KeyStruct {
    turn: Player,
    piece_bitboards: [u64; 6],
    white_piece_bitboard: u64,
    black_piece_bitboard: u64,
    castle_short: [bool; 2],
    castle_long: [bool; 2],
    possible_en_passant: Option<Position>,
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
