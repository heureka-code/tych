use abstraction::ToCppTypes;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl};
use derive_more::{Debug, Display, From, FromStr};

use super::Number1to8;

/// Represents a row of a chess board and is labeld with 1 to 8
#[derive(
    Debug,
    Display,
    FromStr,
    From,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    FromCppTypes,
    ParsingRulesDecl,
)]
#[display("{_0}")]
#[debug("row_{_0}")]
pub struct Row(Number1to8);

impl ToCppTypes<crate::Rule> for Row {
    fn to_cpp_types(&self) -> String {
        format!("row_{}", self.0)
    }
}

impl Row {
    pub fn new(number: Number1to8) -> Self {
        Self(number)
    }
    /// Getter for the wrapped `u8` label
    pub fn value(&self) -> Number1to8 {
        self.0
    }
    // Getter for the label as `u8`
    pub fn number(&self) -> u8 {
        *self.value()
    }
    // Getter for the index of the row (0 to 7)
    pub fn index(&self) -> u8 {
        self.value().index()
    }
}

impl TryFrom<char> for Row {
    type Error = <Number1to8 as TryFrom<char>>::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}
