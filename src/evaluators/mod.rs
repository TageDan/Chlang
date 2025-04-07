use crate::tree_evaluator::{self, Eval};

pub mod material_evaluator;

pub mod evaluator_0;

pub mod positional_evaluator;

#[derive(Clone)]
pub struct NoneEvaluator;

impl tree_evaluator::Eval for NoneEvaluator {
    fn evaluate(&self, board: &mut crate::board::Board) -> isize {
        return 0;
    }
    fn modified(&self) -> Box<dyn Eval + Sync + Send> {
        return Box::new(self.clone());
    }
    fn bot_clone(&self) -> Box<dyn Eval + Sync + Send> {
        Box::new(self.clone())
    }
    fn string_rep(&self) -> String {
        String::from("")
    }
}
