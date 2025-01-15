use board::{GameState, Player, Position};
use cmove::Move;
use piece::Piece;
use std::io::BufRead;

#[cfg(feature = "gui")]
use pix_engine::prelude::*;

mod board;
pub mod cmove;
pub mod piece;

#[cfg(feature = "gui")]
struct Game {
    board: board::Board,
    selected: Option<Position>,
    state: GameState,
    white_piece_images: [Image; 6],
    black_piece_images: [Image; 6],
}

#[cfg(feature = "gui")]
impl PixEngine for Game {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.image_mode(ImageMode::Center);
        s.font_size(25)?;
        s.rect_mode(RectMode::Center);
        s.blend_mode(BlendMode::Blend);

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
                Point::from_xy(pos.col as i32 * 50 + 75, pos.row as i32 * 50 + 75),
                50
            ])?;

            s.stroke(Color::BLACK);
            s.fill(Color::WHITE);

            let (x, y) = (pos.col as i32 * 50 + 75, pos.row as i32 * 50 + 75);
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

        match self.state {
            GameState::Playing => (),
            GameState::Win(board::Player::White) => {
                s.fill(Color::BLACK);
                s.circle(circle![s.center()?, 100])?;
                s.fill(Color::WHITE);
                s.set_cursor_pos(s.center()?);
                s.text("White Wins")?;
                return Ok(());
            }
            GameState::Win(board::Player::Black) => {
                s.fill(Color::BLACK);
                s.circle(circle![s.center()?, 150])?;
                s.fill(Color::WHITE);
                s.set_cursor_pos(s.center()?);
                s.text("Black Wins")?;
                return Ok(());
            }
            GameState::Draw => {
                s.fill(Color::BLACK);
                s.circle(circle![s.center()?, 100])?;
                s.fill(Color::WHITE);
                s.set_cursor_pos(s.center()?);
                s.text("Draw")?;
                return Ok(());
            }
        }

        if s.mouse_clicked(Mouse::Left) {
            let p = s.mouse_pos();

            let x = *p.get(0).unwrap();
            let y = *p.get(1).unwrap();

            let col = (x - 50) / 50;
            let row = (y - 50) / 50;
            if self.selected.is_some()
                && self
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
        };

        Ok(())
    }
}

#[cfg(feature = "gui")]
fn main() -> PixResult<()> {
    use board::Board;

    let mut engine = Engine::builder()
        .dimensions(500, 500)
        .title("chlang")
        .show_frame_rate()
        .build()?;

    let mut app = Game {
        board: Board::default(),
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
    if let Some(path) = a.next() {
        let moves = std::fs::read_to_string(path).unwrap();
        let moves = moves.split("\n");
        for m in moves {
            if m.is_empty() {
                continue;
            }
            let cmove: Move = m.parse().unwrap();
            let res = board.make_move(&cmove.clone());
            if res.is_err() {
                println!("error making move: {:?}", res);
            }
        }
        println!("\x1b[2J\x1b[H");
        if cfg!(not(feature = "gui")) {
            println!("{}", board);
        } else {
            board.display(&mut canvas);
        }
        match board.get_game_state() {
            board::GameState::Draw => {
                println!("DRAW");
            }
            board::GameState::Win(board::Player::White) => {
                println!("White Wins");
            }
            board::GameState::Win(board::Player::Black) => {
                println!("Black Wins");
            }
            _ => (),
        }
        std::process::exit(0);
    }

    let mut stdin = std::io::BufReader::new(std::io::stdin());

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input);
        println!("\x1b[2J\x1b[H");
        if input.trim() == "u" {
            board.unmake_last();
        } else {
            let cmove: Result<Move, &str> = input.parse();

            if cmove.is_ok() {
                let res = board.make_move(&cmove.clone().unwrap());
                if res.is_err() {
                    println!("error making move: {:?}", res);
                }
            }
        }

        #[cfg(feature = "gui")]
        board.display(&mut canvas);

        #[cfg(not(feature = "gui"))]
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
