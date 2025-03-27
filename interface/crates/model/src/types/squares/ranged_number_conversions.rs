use super::Number1to8;
use crate::Rule;
use abstraction::FromCppTypes;
use pest::iterators::Pair;
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum NumberNotInRange1To8 {
    #[error("parsing error: {0}")]
    NoU8(<u8 as FromStr>::Err),

    #[error("number out of range: {0}")]
    NumberOutOfRange(u8),
}

impl FromStr for Number1to8 {
    type Err = NumberNotInRange1To8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u8 = s.parse().map_err(NumberNotInRange1To8::NoU8)?;
        num.try_into()
    }
}

impl FromCppTypes<Rule> for Number1to8 {
    fn _try_from_cpp(p: Pair<'_, Rule>) -> Option<Self> {
        p.as_str().parse().ok()
    }
}

impl TryFrom<u8> for Number1to8 {
    type Error = NumberNotInRange1To8;
    fn try_from(number: u8) -> Result<Self, Self::Error> {
        Self::new(number).ok_or(NumberNotInRange1To8::NumberOutOfRange(number))
    }
}
impl TryFrom<char> for Number1to8 {
    type Error = NumberNotInRange1To8;
    fn try_from(number: char) -> Result<Self, Self::Error> {
        number.to_string().parse()
    }
}
