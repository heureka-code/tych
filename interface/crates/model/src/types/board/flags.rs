use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::CastlingRight;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    FromCppTypes,
    ParsingRulesDecl,
    Getters,
    ToCppTypes,
)]
pub struct SituationFlags {
    castling_right: CastlingRightFlags,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    FromCppTypes,
    ParsingRulesDecl,
    Getters,
    ToCppTypes,
)]
pub struct CastlingRightFlags {
    white_right: CastlingRight,
    black_right: CastlingRight,
}
