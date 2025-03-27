use std::str::FromStr;

use super::piece_kind::ProvidedIsNoValidPiece;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::{
    squares::SquareCantBeBuildFromTuple, ColoredPiece, Kind, LetterAToH, Number1to8, Square,
};

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
#[debug("{}{start}{}{destination}{}", piece.short(), self.is_capture_string(), self.promotion_string())]
pub struct NormalMove {
    piece: ColoredPiece,
    start: Square,
    is_capture: bool,
    destination: Square,
    promotion_target: Option<Kind>,
}

#[derive(Debug, thiserror::Error)]
pub enum NormalMoveParsingError {
    #[error("Given string was too short and can't be a move")]
    TooShort,
    #[error("Invalid piece: {0}")]
    InvalidPiece(#[from] ProvidedIsNoValidPiece),
    #[error("Invalid square")]
    InvalidSquare(
        SquareCantBeBuildFromTuple<
            <LetterAToH as TryFrom<char>>::Error,
            <Number1to8 as TryFrom<char>>::Error,
        >,
    ),
}

impl FromStr for NormalMove {
    type Err = NormalMoveParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.chars().peekable();
        let piece = parts.next().ok_or(NormalMoveParsingError::TooShort)?;
        let start_file = parts.next().ok_or(NormalMoveParsingError::TooShort)?;
        let start_row = parts.next().ok_or(NormalMoveParsingError::TooShort)?;
        let maybe_capture = parts.peek().ok_or(NormalMoveParsingError::TooShort)?;
        let is_capture = if *maybe_capture == 'x' {
            parts.next();
            true
        } else {
            false
        };
        let dest_file = parts.next().ok_or(NormalMoveParsingError::TooShort)?;
        let dest_row = parts.next().ok_or(NormalMoveParsingError::TooShort)?;
        let promotion_target = parts.next();
        Ok(Self {
            piece: piece.try_into()?,
            start: Square::try_from((start_file, start_row))
                .map_err(NormalMoveParsingError::InvalidSquare)?,
            is_capture,
            destination: Square::try_from((dest_file, dest_row))
                .map_err(NormalMoveParsingError::InvalidSquare)?,
            promotion_target: promotion_target.map(|pt| pt.try_into()).transpose()?,
        })
    }
}

impl NormalMove {
    #[allow(unused)]
    pub fn is_capture_string(&self) -> String {
        self.is_capture()
            .then_some("x")
            .unwrap_or_default()
            .to_string()
    }
    #[allow(unused)]
    pub fn promotion_string(&self) -> String {
        self.promotion_target()
            .map(|t| t.color_short(self.piece().color()).to_string())
            .unwrap_or_default()
    }
}
