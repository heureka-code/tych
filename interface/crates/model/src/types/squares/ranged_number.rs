use abstraction_derive::ParsingRulesDecl;
use derive_more::{Debug, Deref, Display};

/// `u8` that is checked to be one of `12345678`
///
/// Implements `Deref` and `Copy` so it can be used just like `u8`
#[derive(Debug, Display, Deref, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, ParsingRulesDecl)]
#[display("{_0}")]
#[debug("{_0}")]
pub struct Number1to8(u8);

impl Number1to8 {
    pub fn new(num: u8) -> Option<Self> {
        (1..=8).contains(&num).then_some(num).map(Number1to8)
    }
    /// The wrapped number as u8
    pub fn value(&self) -> u8 {
        self.0
    }
    /// The index of the inner number in `12345678`, always one of 0, 1, 2, 3, 4, 5, 6, 7
    pub fn index(&self) -> u8 {
        self.value() - 1
    }
    /// Iterator from 1 to 8 (ascending order) wrapped inside this type
    pub fn forward_iter() -> impl DoubleEndedIterator<Item = Number1to8> {
        (1..=8).flat_map(Self::new)
    }
}
