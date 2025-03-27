use abstraction_derive::ParsingRulesDecl;
use derive_more::{Debug, Deref, Display};
use itertools::Itertools;

/// `char` that is checked to be one of `abcdefgh` (only lowercase)
///
/// Implements `Deref` and `Copy` so it can be used just like `char`
#[derive(
    Debug, Display, Deref, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, ParsingRulesDecl,
)]
#[display("{_0}")]
#[debug("{_0}")]
pub struct LetterAToH(char);

impl LetterAToH {
    pub fn new(letter: char) -> Option<Self> {
        ('a'..='h')
            .contains(&letter)
            .then_some(letter)
            .map(LetterAToH)
    }
    /// The wrapped letter as `char`
    pub fn value(&self) -> char {
        self.0
    }
    /// The index of the inner character in `abcdefgh`, always one of 0, 1, 2, 3, 4, 5, 6, 7
    pub fn index(&self) -> u8 {
        ('a'..='h')
            .find_position(|x| x == &self.value())
            .map(|l| l.0.try_into().ok())
            .flatten()
            .expect("Safe because inner value is guaranteed to be in range")
    }
    /// Iterator from 'a' to 'h' (ascending order) wrapped inside this type
    pub fn forward_iter() -> impl DoubleEndedIterator<Item = Self> {
        ('a'..='h').flat_map(Self::new)
    }
}
