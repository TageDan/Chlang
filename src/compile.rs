struct State {
    state_type: StateType,
    stage_piece: Stage,
    extra_stage: ExtraStage,
}

enum StateType {
    Extra,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum ExtraStage {
    ShortCastle,
    LongCastle,
}

enum Stage {
    Base,
    Position,
    Attack,
    Moves,
}

struct PieceValues {
    base: u8,
    position: Vec<u8>,
    attack: u8,
    moves: u8,
}

struct Parsed {
    state: State,
    extra: [u8; 2],
    pawn: PieceValues,
    knight: PieceValues,
    bishop: PieceValues,
    rook: PieceValues,
    queen: PieceValues,
    king: PieceValues,
}

pub fn compile(source: String) -> String {
    let mut parsed = Parsed {
        state: State {
            state_type: StateType::Extra,
            stage_piece: Stage::Base,
            extra_stage: ExtraStage::ShortCastle,
        },
        extra: [0, 0],
        pawn: PieceValues {
            base: 0,
            position: Vec::with_capacity(64),
            attack: 0,
            moves: 0,
        },
        knight: PieceValues {
            base: 0,
            position: Vec::with_capacity(64),
            attack: 0,
            moves: 0,
        },
        bishop: PieceValues {
            base: 0,
            position: Vec::with_capacity(64),
            attack: 0,
            moves: 0,
        },
        rook: PieceValues {
            base: 0,
            position: Vec::with_capacity(64),
            attack: 0,
            moves: 0,
        },
        queen: PieceValues {
            base: 0,
            position: Vec::with_capacity(64),
            attack: 0,
            moves: 0,
        },
        king: PieceValues {
            base: 0,
            position: Vec::with_capacity(64),
            attack: 0,
            moves: 0,
        },
    };
    for line in source.lines() {
        let trimmed = line.trim();
        match trimmed.to_lowercase().as_str() {
            "extra:" => {
                parsed.state.state_type = StateType::Extra;
                continue;
            }
            "pawn:" => {
                parsed.state.state_type = StateType::Pawn;
                continue;
            }
            "knight:" => {
                parsed.state.state_type = StateType::Knight;
                continue;
            }
            "bishop:" => {
                parsed.state.state_type = StateType::Bishop;
                continue;
            }
            "rook:" => {
                parsed.state.state_type = StateType::Rook;
                continue;
            }
            "queen:" => {
                parsed.state.state_type = StateType::Queen;
                continue;
            }
            "king:" => {
                parsed.state.state_type = StateType::King;
                continue;
            }
            "longcastle:" => match parsed.state.state_type {
                StateType::Extra => {
                    parsed.state.extra_stage = ExtraStage::LongCastle;
                    continue;
                }
                _ => todo!(),
            },
            "shortcastle:" => match parsed.state.state_type {
                StateType::Extra => {
                    parsed.state.extra_stage = ExtraStage::ShortCastle;
                    continue;
                }
                _ => todo!(),
            },
            "base:" => match parsed.state.state_type {
                StateType::Extra => todo!(),
                _ => {
                    parsed.state.stage_piece = Stage::Base;
                    continue;
                }
            },
            "position:" => match parsed.state.state_type {
                StateType::Extra => todo!(),
                _ => {
                    parsed.state.stage_piece = Stage::Position;
                    continue;
                }
            },
            "attack:" => match parsed.state.state_type {
                StateType::Extra => todo!(),
                _ => {
                    parsed.state.stage_piece = Stage::Attack;
                    continue;
                }
            },
            "moves:" => match parsed.state.state_type {
                StateType::Extra => todo!(),
                _ => {
                    parsed.state.stage_piece = Stage::Moves;
                    continue;
                }
            },
            _ => (),
        }

        if trimmed.chars().all(|c| c.is_numeric()) {
            let val = trimmed.parse::<u8>().unwrap();
            if val > 94 {
                panic!("Too high number");
            }
            match parsed.state.state_type {
                StateType::Extra => match parsed.state.extra_stage {
                    ExtraStage::ShortCastle => parsed.extra[0] = val,
                    ExtraStage::LongCastle => parsed.extra[1] = val,
                },
                StateType::Pawn => match parsed.state.stage_piece {
                    Stage::Base => parsed.pawn.base = val,
                    Stage::Attack => parsed.pawn.attack = val,
                    Stage::Moves => parsed.pawn.moves = val,
                    Stage::Position => todo!(),
                },
                StateType::Knight => match parsed.state.stage_piece {
                    Stage::Base => parsed.knight.base = val,
                    Stage::Attack => parsed.knight.attack = val,
                    Stage::Moves => parsed.knight.moves = val,
                    Stage::Position => todo!(),
                },
                StateType::Bishop => match parsed.state.stage_piece {
                    Stage::Base => parsed.bishop.base = val,
                    Stage::Attack => parsed.bishop.attack = val,
                    Stage::Moves => parsed.bishop.moves = val,
                    Stage::Position => todo!(),
                },
                StateType::Rook => match parsed.state.stage_piece {
                    Stage::Base => parsed.rook.base = val,
                    Stage::Attack => parsed.rook.attack = val,
                    Stage::Moves => parsed.rook.moves = val,
                    Stage::Position => todo!(),
                },
                StateType::Queen => match parsed.state.stage_piece {
                    Stage::Base => parsed.queen.base = val,
                    Stage::Attack => parsed.queen.attack = val,
                    Stage::Moves => parsed.queen.moves = val,
                    Stage::Position => todo!(),
                },
                StateType::King => match parsed.state.stage_piece {
                    Stage::Base => parsed.king.base = val,
                    Stage::Attack => parsed.king.attack = val,
                    Stage::Moves => parsed.king.moves = val,
                    Stage::Position => todo!(),
                },
            }
            continue;
        }
        let new_trimmed = trimmed.chars().fold(String::new(), |mut acc, x| {
            if acc.chars().last().is_some_and(|x| x == ' ') && x == ' ' {
                return acc;
            }
            acc.push(x);
            acc
        });
        let nums = new_trimmed.split(" ");
        for snum in nums {
            let num_inter = snum.parse::<u8>();
            let num = if num_inter.is_ok() {
                num_inter.unwrap()
            } else {
                panic!("{}", snum);
            };

            match parsed.state.stage_piece {
                Stage::Position => (),
                _ => todo!(),
            }
            match parsed.state.state_type {
                StateType::Extra => todo!(),
                StateType::Pawn => parsed.pawn.position.push(num),
                StateType::Knight => parsed.knight.position.push(num),
                StateType::Bishop => parsed.bishop.position.push(num),
                StateType::Rook => parsed.rook.position.push(num),
                StateType::Queen => parsed.queen.position.push(num),
                StateType::King => parsed.king.position.push(num),
            }
        }
    }

    if parsed.pawn.position.len() != 64
        || parsed.knight.position.len() != 64
        || parsed.bishop.position.len() != 64
        || parsed.rook.position.len() != 64
        || parsed.queen.position.len() != 64
        || parsed.king.position.len() != 64
    {
        panic!()
    }
    let mut bytes = Vec::new();
    bytes.push(parsed.pawn.base);
    bytes.push(parsed.knight.base);
    bytes.push(parsed.bishop.base);
    bytes.push(parsed.rook.base);
    bytes.push(parsed.queen.base);
    bytes.push(parsed.king.base);
    for b in parsed.pawn.position {
        bytes.push(b);
    }
    for b in parsed.knight.position {
        bytes.push(b);
    }
    for b in parsed.bishop.position {
        bytes.push(b);
    }
    for b in parsed.rook.position {
        bytes.push(b);
    }
    for b in parsed.queen.position {
        bytes.push(b);
    }
    for b in parsed.king.position {
        bytes.push(b);
    }
    bytes.push(parsed.pawn.attack);
    bytes.push(parsed.knight.attack);
    bytes.push(parsed.bishop.attack);
    bytes.push(parsed.rook.attack);
    bytes.push(parsed.queen.attack);
    bytes.push(parsed.king.attack);
    bytes.push(parsed.extra[0]);
    bytes.push(parsed.extra[1]);
    bytes.push(parsed.pawn.moves);
    bytes.push(parsed.knight.moves);
    bytes.push(parsed.bishop.moves);
    bytes.push(parsed.rook.moves);
    bytes.push(parsed.queen.moves);
    bytes.push(parsed.king.moves);

    bytes.iter().map(|x| (x + 33) as char).collect::<String>()
}

fn decompile(str: &str) {
    todo!();
    let mut result = String::new();
    let mut bytes = str.bytes().map(|x| x - 33).collect::<Vec<u8>>();

    result += "Extra:
        LongCastle:
             
    ";
}
