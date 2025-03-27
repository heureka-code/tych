use abstraction_derive::ToCppTypes;
use derive_getters::Getters;

use super::MoveContainer;

/// Marks a list of moves so that it should get executed on an initial chess board
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, ToCppTypes, Getters)]
#[allow(non_camel_case_types)]
pub struct execute_on_initial {
    moves: MoveContainer,
}
impl execute_on_initial {
    pub fn new(m: MoveContainer) -> Self {
        Self { moves: m }
    }
}
