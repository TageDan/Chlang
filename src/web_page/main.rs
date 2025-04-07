use chlang::{
    board::{self, Board, GameState, Player, Position},
    cmove::Move,
    evaluators::evaluator_0,
    piece::Piece,
    tree_evaluator::Bot,
    User,
};
use leptos::{html::Time, logging, prelude::*, task::spawn_local};
use rustc_hash::FxHashMap;

fn main() {
    leptos::mount::mount_to_body(App)
}

fn check_and_update(
    string: String,
    set_board: WriteSignal<board::Board>,
    board: ReadSignal<Board>,
    write_white_player: WriteSignal<User>,
    white_player: ReadSignal<User>,
    black_player: ReadSignal<User>,
) {
    let bytes = string.as_bytes();
    let bot = evaluator_0::Evaluator::from(bytes);
    let bot = Bot {
        evaluator: Box::new(bot),
        search_depth: 3,
        cache: FxHashMap::default(),
    };
    let b = Board::default();
    let u = User::Bot(bot);
    set_board.set(b);
    write_white_player.set(u);
    play(white_player, black_player, board, set_board);
}

fn handle_click_on_board(
    selected_square: ReadSignal<Option<Position>>,
    set_selected_square: WriteSignal<Option<Position>>,
    board: ReadSignal<Board>,
    set_board: WriteSignal<Board>,
    square_idx: i64,
    white_player: ReadSignal<User>,
    black_player: ReadSignal<User>,
) {
    if selected_square
        .get()
        .is_some_and(|p| p.col + p.row * 8 == square_idx)
    {
        *set_selected_square.write() = None
    } else if let Some(p1) = selected_square.get() {
        let p2 = Position::new(square_idx / 8, square_idx % 8);
        let mut b = board.get();
        let res = b.make_move(&Move::new(&p1, &p2));
        if res.is_err() {
            let pos = Position::new(square_idx / 8, square_idx % 8);
            *set_selected_square.write() = Some(pos);
        } else {
            *set_board.write() = b;
            *set_selected_square.write() = None;
            play(white_player, black_player, board, set_board);
        }
    } else {
        let pos = Position::new(square_idx / 8, square_idx % 8);
        *set_selected_square.write() = Some(pos);
    }
}

fn play(
    white_player: ReadSignal<User>,
    black_player: ReadSignal<User>,
    board: ReadSignal<Board>,
    set_board: WriteSignal<board::Board>,
) {
    match board.get().turn {
        Player::White => match white_player.get() {
            User::Human => return,
            User::Bot(mut b) => {
                spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(50).await;
                    let cmove = b.find_best_move(&mut board.get()).unwrap();
                    let mut b = board.get();
                    b.make_move(&cmove);
                    set_board.set(b);
                    // play(white_player, black_player, board, set_board);
                });
            }
        },
        Player::Black => match black_player.get() {
            User::Human => return,
            User::Bot(mut b) => {
                spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(50).await;
                    let cmove = b.find_best_move(&mut board.get()).unwrap();
                    let mut b = board.get();
                    b.make_move(&cmove);
                    set_board.set(b);
                    // play(white_player, black_player, board, set_board);
                });
            }
        },
    }
}

fn random(set_string: WriteSignal<String>) {
    let mut s = String::new();
    for _ in 0..398 {
        let mut c = 0;
        while !(33..=126).contains(&c) {
            c = rand::random::<u8>();
        }
        s.push(char::from(c));
    }
    set_string.set(s);
}

#[component]
fn App() -> impl IntoView {
    let (board, set_board) = signal(board::Board::default());
    let (string, set_string) = signal(String::new());
    let (selected_square, set_selected_square) = signal::<Option<Position>>(None);

    let (black_player, set_black_player) = signal(User::Human);
    let (white_player, set_white_player) = signal(User::Human);
    let game_state = move || board.get().get_game_state();

    let form_or_game = move || {
        view! {

        <div class="top-bar">
            <label for="stringrep">String representation / id</label>
            <input id="string_rep" maxlength="398" minlength="398" required bind:value=(string,set_string) />
            <button on:click= move|_| check_and_update(string.get(), set_board, board, set_white_player, white_player, black_player) >play</button>
            <button on:click= move|_| random(set_string)>randomize</button>
        </div>


        <div class="board" >
            {(0..64).into_iter().map(|n| view! {
                <div
                class:gridodd=move || (n + n/8) %2 == 1
                class:grideven=move || (n+n/8) %2 == 0
                class:gridselected=move || selected_square.get().is_some_and(|x| x.row*8 + x.col == n)

                on:click=move|_| handle_click_on_board(selected_square, set_selected_square, board,set_board, n, white_player, black_player)

                inner_html={

                    move || match board.get().piece_type(&Position::from(1<<n)) {
                        Some((color, piece)) => {
                            match color {
                                Player::White => {
                                    match piece {
                                        Piece::King =>
                                            "<img src='images/white-king.png' />"
                                        ,
                                        Piece::Queen =>
                                        "<img src='images/white-queen.png' />"
                                        ,
                                        Piece::Knight =>
                                            "<img src='images/white-knight.png' />"
                                        ,
                                        Piece::Pawn =>
                                            "<img src='images/white-pawn.png' />"
                                        ,
                                        Piece::Bishop =>
                                            "<img src='images/white-bishop.png' />"
                                        ,
                                        _ =>
                                            "<img src='images/white-rook.png' />"

                                    }
                                },
                                Player::Black => {
                                    match piece {
                                        Piece::King =>
                                            "<img src='images/black-king.png' />"
                                        ,
                                        Piece::Queen =>
                                        "<img src='images/black-queen.png' />"
                                        ,
                                        Piece::Knight =>
                                            "<img src='images/black-knight.png' />"
                                        ,
                                        Piece::Pawn =>
                                            "<img src='images/black-pawn.png' />"
                                        ,
                                        Piece::Bishop =>
                                            "<img src='images/black-bishop.png' />"
                                        ,
                                        _ =>
                                            "<img src='images/black-rook.png' />"

                                    }
                                }
                            }
                        },
                        None => "",
                    }
                }></div>
            }).collect::<Vec<_>>()}
        </div>

        <h3>GameState</h3>
        <p>{move || match game_state(){
            GameState::Playing => "PLAYING",
            GameState::Draw => "DRAW",
            GameState::Win(Player::White) => "White wins",
            GameState::Win(Player::Black) => "Black Wins"
        }}</p>


        <div>
        <h1>Chlang</h1>
        Hi there! This is the chlang web interface. Here you can play chess localy against a friend or insert a string representation of a bot ( the id ) and submit it at the top of the screen. The id must be precisely 398 chars long and can only contain <a href="https://www.ibm.com/docs/en/sdse/6.4.0?topic=configuration-ascii-characters-from-33-126"> ascii-printable chars</a>. This means there are 94^398 different possible bots. That is a lot of bots :O!!!!
        </div>
        }
    };

    view! {
        {form_or_game}
    }
}
