use abstraction::ToCppTypes;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl};
use derive_more::{Debug, Display, From, FromStr};

use super::LetterAToH;

/// Represents a file (column) of a chess board and is labeld with 'a' to 'h'
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
#[debug("file_{_0}")]
pub struct File(LetterAToH);

impl File {
    pub fn new(letter: LetterAToH) -> File {
        Self(letter)
    }
    // Getter for the wrapped character representing the label
    pub fn value(&self) -> LetterAToH {
        self.0
    }
    // Getter for the character representing the label
    pub fn letter(&self) -> char {
        *self.value()
    }
    // Getter for the index (0 to 7) of the file
    pub fn index(&self) -> u8 {
        self.value().index()
    }
}

impl ToCppTypes<crate::Rule> for File {
    fn to_cpp_types(&self) -> String {
        format!("file_{}", self.0)
    }
}

impl TryFrom<char> for File {
    type Error = <LetterAToH as TryFrom<char>>::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}
