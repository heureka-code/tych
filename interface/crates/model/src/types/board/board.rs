use abstraction_derive::{ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;
use itertools::Itertools;

use super::{PlacedPiece, Rule, Square};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, ParsingRulesDecl, Getters, ToCppTypes)]
#[debug("Board {{ pieces: {} }}", format!("{:?}", pieces))]
pub struct Board {
    pieces: Vec<PlacedPiece>,
}

impl Board {
    pub fn new(pieces: Vec<PlacedPiece>) -> Self {
        Self { pieces }
    }
    pub fn get_by_square<'a>(&'a self, square: &Square) -> Option<&'a PlacedPiece> {
        self.pieces()
            .iter()
            .filter(|p| p.square() == square)
            .exactly_one()
            .ok()
    }
}

impl abstraction::FromCppTypes<Rule> for Board {
    fn _try_from_cpp(p: pest::iterators::Pair<'_, crate::Rule>) -> Option<Self> {
        let pieces = p
            .into_inner()
            .map(PlacedPiece::try_from_cpp)
            .collect::<Option<Vec<_>>>()?;
        Some(Board { pieces })
    }
}
