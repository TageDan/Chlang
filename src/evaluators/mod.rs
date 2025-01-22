use crate::tree_evaluator;

pub mod material_evaluator;

pub mod positional_evaluator;

pub struct NoneEvaluator;

impl tree_evaluator::Eval for NoneEvaluator {
    fn evaluate(&self, board: &mut crate::board::Board) -> isize {
        return 0;
    }
}
