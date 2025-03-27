use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::{piece_kind::ProvidedIsNoValidPiece, Color, Kind, Square};

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
    Getters,
    FromCppTypes,
    ParsingRulesDecl,
    ToCppTypes,
)]
#[debug("{color}{kind}")]
pub struct ColoredPiece {
    color: Color,
    kind: Kind,
}
impl ColoredPiece {
    pub fn short(&self) -> char {
        self.kind().color_short(self.color())
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
    Getters,
    ToCppTypes,
)]
#[debug("{}_{}", square, piece.short())]
pub struct PlacedPiece {
    piece: ColoredPiece,
    square: Square,
}

impl TryFrom<char> for ColoredPiece {
    type Error = ProvidedIsNoValidPiece;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            color: if value.is_ascii_uppercase() {
                Color::White
            } else {
                Color::Black
            },
            kind: value.try_into()?,
        })
    }
}
