use std::str::FromStr;

use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;

use super::Color;

#[derive(Debug, derive_more::Display, thiserror::Error)]
#[display("InvalidCastlingDeclaration")]
pub struct InvalidCastlingDeclaration;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, FromCppTypes, ParsingRulesDecl, ToCppTypes,
)]
pub enum CastlingKind {
    Kingside,
    Queenside,
}

impl FromStr for CastlingKind {
    type Err = InvalidCastlingDeclaration;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0-0" => Self::Kingside,
            "0-0-0" => Self::Queenside,
            _ => Err(InvalidCastlingDeclaration)?,
        })
    }
}
impl CastlingKind {
    pub fn as_short_move_str(&self) -> &'static str {
        match self {
            Self::Kingside => "0-0",
            Self::Queenside => "0-0-0",
        }
    }
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
    ToCppTypes,
    Getters,
    derive_more::FromStr,
)]
pub struct CastlingMove {
    castling_kind: CastlingKind,
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
    ToCppTypes,
    Getters,
)]
pub struct ColoredCastlingMove {
    color: Color,
    castling_move: CastlingMove,
}
