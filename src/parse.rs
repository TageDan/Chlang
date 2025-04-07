use std::{env::Args, error::Error, str::FromStr};

use rustc_hash::FxHashMap;

use crate::{evaluators, tree_evaluator, User};

pub fn parse<T>(a: &mut T) -> Result<User, Box<dyn Error>>
where
    T: Iterator<Item = String>,
{
    if let Some(s) = a.next() {
        match s.as_str() {
            "HUMAN" => Ok(User::Human),
            "MATERIAL" => Ok(User::Bot(tree_evaluator::Bot {
                evaluator: Box::new(
                    #[cfg(feature = "using_default")]
                    evaluators::material_evaluator::MaterialEvaluator::default(),
                    #[cfg(not(feature = "using_default"))]
                    evaluators::material_evaluator::MaterialEvaluator::from_str(
                        &a.next().expect("insert string representation"),
                    )?,
                ),
                search_depth: a
                    .next()
                    .expect("Please insert search depth for MATERIAL bot")
                    .parse::<u8>()
                    .expect("Invalid search depth: must be a valid u8"),
                cache: FxHashMap::default(),
            })),
            "POSITIONAL" => Ok(User::Bot(tree_evaluator::Bot {
                search_depth: a
                    .next()
                    .expect("Please insert search depth for MATERIAL bot")
                    .parse::<u8>()
                    .expect("Invalid search depth: must be a valid u8"),
                evaluator: Box::new(
                    #[cfg(feature = "using_default")]
                    evaluators::positional_evaluator::PositionalEvaluator::default(),
                    #[cfg(not(feature = "using_default"))]
                    evaluators::positional_evaluator::PositionalEvaluator::from_str(
                        &a.next()
                            .expect("insert string representation of eval function"),
                    )?,
                ),
                cache: FxHashMap::default(),
            })),
            "RANDOM" => Ok(User::Bot(tree_evaluator::Bot {
                evaluator: Box::new(evaluators::NoneEvaluator),
                search_depth: 1,
                cache: FxHashMap::default(),
            })),

            "DEFAULT" => Ok(User::Bot(tree_evaluator::Bot {
                evaluator: Box::new(evaluators::evaluator_0::Evaluator::default()),
                search_depth: 4,
                cache: FxHashMap::default(),
            })),

            s => Ok(User::Bot(tree_evaluator::Bot {
                evaluator: Box::new(
                    evaluators::evaluator_0::Evaluator::from_str(s).expect("Bad string rep"),
                ),
                search_depth: 4,
                cache: FxHashMap::default(),
            })),
        }
    } else {
        Ok(User::Human)
    }
}
