use std::str::FromStr;

use super::LetterAToH;
use crate::Rule;

use abstraction::FromCppTypes;
use pest::iterators::Pair;

#[derive(thiserror::Error, Debug)]
pub enum LetterNotInRangeAToH {
    #[error("not a single character")]
    NoChar(<char as FromStr>::Err),

    #[error("letter out of range: {0}")]
    NotAToH(char),
}

impl FromStr for LetterAToH {
    type Err = LetterNotInRangeAToH;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter: char = s.parse().map_err(LetterNotInRangeAToH::NoChar)?;
        letter.try_into()
    }
}

impl FromCppTypes<Rule> for LetterAToH {
    fn _try_from_cpp(p: Pair<'_, Rule>) -> Option<Self> {
        p.as_str().parse().ok()
    }
}

impl TryFrom<char> for LetterAToH {
    type Error = LetterNotInRangeAToH;
    fn try_from(letter: char) -> Result<Self, Self::Error> {
        Self::new(letter).ok_or(LetterNotInRangeAToH::NotAToH(letter))
    }
}
