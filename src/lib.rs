pub mod board;
pub mod cmove;
pub mod compile;
pub mod evaluators;
pub mod game;
pub mod parse;
pub mod piece;
pub mod tree_evaluator;
#[derive(Clone)]
pub enum User {
    Human,
    Bot(tree_evaluator::Bot),
}
