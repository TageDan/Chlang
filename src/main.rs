use board::{GameState, Player, Position};
use cmove::Move;
use piece::Piece;
use std::io::BufRead;
use tree_evaluator::Eval;

#[cfg(feature = "gui")]
use pix_engine::prelude::*;

mod board;
pub mod cmove;
pub mod evaluators;
pub mod piece;
pub mod tree_evaluator;

enum User {
    Human,
    Bot(tree_evaluator::Bot),
}

#[cfg(feature = "gui")]
struct Game {
    board: board::Board,
    selected: Option<Position>,
    state: GameState,
    white_piece_images: [Image; 6],
    black_piece_images: [Image; 6],
    black_player: User,
    white_player: User,
    promotion: Option<Position>,
}

#[cfg(feature = "gui")]
impl PixEngine for Game {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.image_mode(ImageMode::Center);
        s.font_size(25)?;
        s.rect_mode(RectMode::Center);
        s.blend_mode(BlendMode::Blend);

        match self.state {
            GameState::Playing => (),
            _ => return self.draw(s),
        }

        if s.key_down(Key::Tab) {
            println!(
                "material eval: {}",
                evaluators::material_evaluator::MaterialEvaluator::default()
                    .evaluate(&mut self.board)
            );
            return self.draw(s);
        }

        // If current player is bot then let it play
        match self.board.turn {
            Player::White => match self.white_player {
                User::Human => (),
                User::Bot(ref b) => {
                    let cmove = b.find_best_move(&mut self.board);
                    if let Some(cmove) = cmove {
                        self.board
                            .make_move(&cmove)
                            .expect("bot made unvalid move?!");
                    }
                    self.state = self.board.get_game_state();
                    return self.draw(s);
                }
            },
            Player::Black => match self.black_player {
                User::Human => (),
                User::Bot(ref b) => {
                    let cmove = b.find_best_move(&mut self.board);
                    if let Some(cmove) = cmove {
                        self.board
                            .make_move(&cmove)
                            .expect("bot made unvalid move?!");
                    }
                    self.state = self.board.get_game_state();
                    return self.draw(s);
                }
            },
        }

        if s.mouse_clicked(Mouse::Left) {
            let p = s.mouse_pos();

            let x = *p.get(0).unwrap();
            let y = *p.get(1).unwrap();

            if let Some(ref pro_pos) = self.promotion {
                if y > s.height()? as i32 / 2 - 20
                    && y < s.height()? as i32 / 2 + 20
                    && x > s.width()? as i32 / 2 - 80
                    && x < s.width()? as i32 / 2 + 80
                {
                    let col = (x + 80 - s.width()? as i32 / 2) as usize / 40;
                    let promotion = match col {
                        3 => Piece::Knight,
                        2 => Piece::Bishop,
                        1 => Piece::Rook,
                        0 => Piece::Queen,
                        _ => {
                            return Err(
                                PixError::Renderer(String::from("Invalid promotion piece")).into()
                            )
                        }
                    };
                    if self
                        .board
                        .make_move(&Move::promotion(
                            self.selected.as_ref().unwrap(),
                            pro_pos,
                            promotion,
                        ))
                        .is_ok()
                    {
                        self.selected = None;
                        self.promotion = None;
                        return self.draw(s);
                    }
                } else {
                    self.selected = None;
                    self.promotion = None;
                    return self.draw(s);
                }
            };

            let col = (x - 50) / 50;
            let row = (y - 50) / 50;
            if row < 0 || row > 7 || col < 0 || col > 7 {
                return self.draw(s);
            }
            let row = 7 - row;
            if self.selected.is_some() {
                let row_i = match self.board.turn {
                    Player::White => 7,
                    Player::Black => 0,
                };
                if self.board.piece_type(&self.selected.clone().unwrap())
                    == Some((self.board.turn.clone(), Piece::Pawn))
                    && row == row_i
                {
                    self.promotion = Some(Position::new(row, col));
                    return self.draw(s);
                }
                if self
                    .board
                    .make_move(&Move::new(
                        &self.selected.clone().unwrap(),
                        &Position::new(row, col),
                    ))
                    .is_ok()
                {
                    self.selected = None;
                    self.state = self.board.get_game_state();
                } else {
                    self.selected = Some(Position::new(row, col));
                }
            } else {
                self.selected = Some(Position::new(row, col));
            }
        };

        self.draw(s)
    }
}

#[cfg(feature = "gui")]
impl Game {
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        // draw board
        for p in 0..64 {
            if p % 2 == (p / 8) % 2 {
                s.fill(Color::DARK_GRAY)
            } else {
                s.fill(Color::GRAY)
            }
            let pos = Position::from(2_u64.pow(p));
            if self.selected.as_ref().is_some_and(|x| *x == pos) {
                s.fill(Color::CYAN)
            }
            s.square(square![
                Point::from_xy(pos.col as i32 * 50 + 75, (7 - pos.row as i32) * 50 + 75),
                50
            ])?;

            s.stroke(Color::BLACK);
            s.fill(Color::WHITE);

            let (x, y) = (pos.col as i32 * 50 + 75, (7 - pos.row as i32) * 50 + 75);
            match self.board.piece_type(&pos) {
                None => (),
                Some((Player::White, p)) => {
                    s.image_transformed(
                        &self.white_piece_images[p.bitboard_index()],
                        None,
                        Some(rect!(point!(x, y), 40, 40)),
                        0.,
                        point!(x, y),
                        Flipped::None,
                    )?;
                }
                Some((Player::Black, p)) => {
                    s.image_transformed(
                        &self.black_piece_images[p.bitboard_index()],
                        None,
                        Some(rect!(point!(x, y), 40, 40)),
                        0.,
                        point!(x, y),
                        Flipped::None,
                    )?;
                }
            }
        }

        if let Some(ref p) = self.promotion {
            s.fill(Color::DARK_GRAY);
            s.rect(rect![s.center()?, 160, 40])?;
            for t in 1..5_i32 {
                let x = (25 - t * 10) * 4 + s.width()? as i32 / 2;
                match p.row {
                    0 => s.image_transformed(
                        &self.black_piece_images[t as usize],
                        None,
                        Some(rect!(point!(x, s.height()? as i32 / 2), 40, 40)),
                        0.,
                        point!(x, s.height()? as i32 / 2),
                        Flipped::None,
                    )?,
                    7 => s.image_transformed(
                        &self.black_piece_images[t as usize],
                        None,
                        Some(rect!(point!(x, s.height()? as i32 / 2), 40, 40)),
                        0.,
                        point!(x, s.height()? as i32 / 2),
                        Flipped::None,
                    )?,
                    _ => {
                        return Err(PixError::Renderer(String::from("Invalid promotion row")).into())
                    }
                }
            }
        }

        match self.state {
            GameState::Playing => (),
            GameState::Win(board::Player::White) => {
                s.fill(Color::rgba(0, 0, 0, 100));
                s.circle(circle![s.center()?, 100])?;
                s.fill(Color::rgba(255, 255, 255, 150));
                s.set_cursor_pos(s.center()?);
                s.text("White Wins")?;
                return Ok(());
            }
            GameState::Win(board::Player::Black) => {
                s.fill(Color::rgba(0, 0, 0, 100));
                s.circle(circle![s.center()?, 150])?;
                s.fill(Color::rgba(255, 255, 255, 150));
                s.set_cursor_pos(s.center()?);
                s.text("Black Wins")?;
                return Ok(());
            }
            GameState::Draw => {
                s.fill(Color::rgba(0, 0, 0, 100));
                s.circle(circle![s.center()?, 100])?;
                s.fill(Color::rgba(255, 255, 255, 150));
                s.set_cursor_pos(s.center()?);
                s.text("Draw")?;
                return Ok(());
            }
        }

        Ok(())
    }
}

#[cfg(feature = "gui")]
impl Default for Game {
    fn default() -> Self {
        Self {
            board: board::Board::default(),
            selected: None,
            state: GameState::Playing,
            white_piece_images: [
                Image::from_file("images/white-pawn.png").unwrap(),
                Image::from_file("images/white-knight.png").unwrap(),
                Image::from_file("images/white-bishop.png").unwrap(),
                Image::from_file("images/white-rook.png").unwrap(),
                Image::from_file("images/white-queen.png").unwrap(),
                Image::from_file("images/white-king.png").unwrap(),
            ],
            black_piece_images: [
                Image::from_file("images/black-pawn.png").unwrap(),
                Image::from_file("images/black-knight.png").unwrap(),
                Image::from_file("images/black-bishop.png").unwrap(),
                Image::from_file("images/black-rook.png").unwrap(),
                Image::from_file("images/black-queen.png").unwrap(),
                Image::from_file("images/black-king.png").unwrap(),
            ],
            black_player: User::Human,
            white_player: User::Human,
            promotion: None,
        }
    }
}

#[cfg(feature = "gui")]
fn main() -> PixResult<()> {
    let mut engine = Engine::builder()
        .dimensions(500, 500)
        .title("chlang")
        .show_frame_rate()
        .build()?;

    let mut args = std::env::args();
    args.next();
    let white_player = if let Some(s) = args.next() {
        match s.as_str() {
            "HUMAN" => User::Human,
            "MATERIAL" => User::Bot(tree_evaluator::Bot {
                search_depth: args
                    .next()
                    .expect("Please insert search depth for MATERIAL bot")
                    .parse::<u8>()
                    .expect("Invalid search depth: must be a valid u8"),
                evaluator: Box::new(evaluators::material_evaluator::MaterialEvaluator::default()),
            }),
            "POSITIONAL" => User::Bot(tree_evaluator::Bot {
                search_depth: args
                    .next()
                    .expect("Please insert search depth for MATERIAL bot")
                    .parse::<u8>()
                    .expect("Invalid search depth: must be a valid u8"),
                evaluator: Box::new(
                    evaluators::positional_evaluator::PositionalEvaluator::default(),
                ),
            }),
            "RANDOM" => User::Bot(tree_evaluator::Bot {
                evaluator: Box::new(evaluators::NoneEvaluator),
                search_depth: 1,
            }),
            err => panic!("Unvalid bot: {err}"),
        }
    } else {
        User::Human
    };

    let black_player = if let Some(s) = args.next() {
        match s.as_str() {
            "HUMAN" => User::Human,
            "MATERIAL" => User::Bot(tree_evaluator::Bot {
                search_depth: args
                    .next()
                    .expect("Please insert search depth for MATERIAL bot")
                    .parse::<u8>()
                    .expect("Invalid search depth: must be a valid u8"),
                evaluator: Box::new(evaluators::material_evaluator::MaterialEvaluator::default()),
            }),
            "POSITIONAL" => User::Bot(tree_evaluator::Bot {
                search_depth: args
                    .next()
                    .expect("Please insert search depth for MATERIAL bot")
                    .parse::<u8>()
                    .expect("Invalid search depth: must be a valid u8"),
                evaluator: Box::new(
                    evaluators::positional_evaluator::PositionalEvaluator::default(),
                ),
            }),
            "RANDOM" => User::Bot(tree_evaluator::Bot {
                evaluator: Box::new(evaluators::NoneEvaluator),
                search_depth: 1,
            }),
            err => panic!("Unvalid bot: {err}"),
        }
    } else {
        User::Human
    };

    let mut app = Game {
        white_player,
        black_player,
        ..Default::default()
    };

    engine.run(&mut app)
}

#[cfg(not(feature = "gui"))]
fn main() {
    /*
    Only for development, shows a stacktrace on stack overflows
    (added since I accidentaly ifinitely called a recursive function.
    Solved by adding a boolean flag to the get_pseudo_legal_king_moves method)
    */
    //unsafe { backtrace_on_stack_overflow::enable() };

    let mut board = board::Board::default();

    let mut a = std::env::args();
    a.next();

    let white_player = {
        if let Some(s) = a.next() {
            match s.as_str() {
                "HUMAN" => User::Human,
                "MATERIAL" => User::Bot(tree_evaluator::Bot {
                    evaluator: Box::new(
                        evaluators::material_evaluator::MaterialEvaluator::default(),
                    ),
                    search_depth: a
                        .next()
                        .expect("Please insert search depth for MATERIAL bot")
                        .parse::<u8>()
                        .expect("Invalid search depth: must be a valid u8"),
                }),
                "POSITIONAL" => User::Bot(tree_evaluator::Bot {
                    search_depth: args
                        .next()
                        .expect("Please insert search depth for MATERIAL bot")
                        .parse::<u8>()
                        .expect("Invalid search depth: must be a valid u8"),
                    evaluator: Box::new(
                        evaluators::positional_evaluator::PositionalEvaluator::default(),
                    ),
                }),
                "RANDOM" => User::Bot(tree_evaluator::Bot {
                    evaluator: Box::new(evaluators::NoneEvaluator),
                    search_depth: 1,
                }),
                _ => panic!("Invalid evaluator"),
            }
        } else {
            User::Human
        }
    };
    let black_player = {
        if let Some(s) = a.next() {
            match s.as_str() {
                "HUMAN" => User::Human,
                "MATERIAL" => User::Bot(tree_evaluator::Bot {
                    evaluator: Box::new(
                        evaluators::material_evaluator::MaterialEvaluator::default(),
                    ),
                    search_depth: a
                        .next()
                        .expect("Please insert search depth for MATERIAL bot")
                        .parse::<u8>()
                        .expect("Invalid search depth: must be a valid u8"),
                }),
                "POSITIONAL" => User::Bot(tree_evaluator::Bot {
                    search_depth: args
                        .next()
                        .expect("Please insert search depth for MATERIAL bot")
                        .parse::<u8>()
                        .expect("Invalid search depth: must be a valid u8"),
                    evaluator: Box::new(
                        evaluators::positional_evaluator::PositionalEvaluator::default(),
                    ),
                }),
                "RANDOM" => User::Bot(tree_evaluator::Bot {
                    evaluator: Box::new(evaluators::NoneEvaluator),
                    search_depth: 1,
                }),
                _ => panic!("Invalid evaluator"),
            }
        } else {
            User::Human
        }
    };

    let mut stdin = std::io::BufReader::new(std::io::stdin());

    loop {
        match board.turn {
            Player::White => match white_player {
                User::Human => {
                    let mut input = String::new();
                    stdin.read_line(&mut input);
                    if input.trim() == "u" {
                        board.unmake_last();
                    } else {
                        let cmove: Result<Move, &str> = input.parse();

                        if cmove.is_ok() {
                            board.make_move(&cmove.clone().unwrap());
                        }
                    }
                }
                User::Bot(ref b) => {
                    let cmove = b.find_best_move(&mut board);
                    if let Some(m) = cmove {
                        board.make_move(&m);
                    }
                }
            },
            Player::Black => match black_player {
                User::Human => {
                    let mut input = String::new();
                    stdin.read_line(&mut input);
                    if input.trim() == "u" {
                        board.unmake_last();
                    } else {
                        let cmove: Result<Move, &str> = input.parse();

                        if cmove.is_ok() {
                            board.make_move(&cmove.clone().unwrap());
                        }
                    }
                }
                User::Bot(ref b) => {
                    let cmove = b.find_best_move(&mut board);
                    if let Some(m) = cmove {
                        board.make_move(&m);
                    }
                }
            },
        }

        println!("\x1b[2J\x1b[H");
        println!("{}", board);

        match board.get_game_state() {
            board::GameState::Draw => {
                println!("DRAW");
                break;
            }
            board::GameState::Win(board::Player::White) => {
                println!("White Wins");
                break;
            }
            board::GameState::Win(board::Player::Black) => {
                println!("Black Wins");
                break;
            }
            _ => (),
        }
    }
}
