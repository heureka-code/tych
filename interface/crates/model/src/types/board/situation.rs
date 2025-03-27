use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::{Board, Color, MoveContainer, SituationFlags};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    ParsingRulesDecl,
    FromCppTypes,
    Getters,
    ToCppTypes,
)]
pub struct Situation {
    board: Board,
    move_history: MoveContainer,
    flags: SituationFlags,
    current_color: Color,
}
