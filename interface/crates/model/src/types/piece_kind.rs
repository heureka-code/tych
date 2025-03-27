use derive_more::{Debug, Display};

use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};

use super::Color;

/// The kind of a piece without color information
#[derive(
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    FromCppTypes,
    ParsingRulesDecl,
    ToCppTypes,
)]
pub enum Kind {
    Pawn,
    King,
    Rook,
    Queen,
    Knight,
    Bishop,
}

impl Kind {
    /// An associated [char] describing the piece kind (one of `pkrqnb`)
    pub fn lowercase_short(&self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::King => 'k',
            Self::Rook => 'r',
            Self::Queen => 'q',
            Self::Knight => 'n',
            Self::Bishop => 'b',
        }
    }
    /// Uses the short letters from [Self::lowercase_short] but takes an additional [Color] and
    /// returns the associated [char] in uppercase if the [Color] is [Color::White] or in lowercase
    /// if it is [Color::Black]
    pub fn color_short(&self, color: &Color) -> char {
        let mut k = self.lowercase_short();
        match color {
            Color::White => k.make_ascii_uppercase(),
            Color::Black => k.make_ascii_lowercase(),
        }
        k
    }
}

abstraction::impl_opt_rule!(Kind);

#[derive(Debug, derive_more::Display, thiserror::Error)]
#[display("ProvidedIsNoValidPiece({_0})")]
pub struct ProvidedIsNoValidPiece(char);

impl TryFrom<char> for Kind {
    type Error = ProvidedIsNoValidPiece;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'P' | 'p' => Kind::Pawn,
            'R' | 'r' => Kind::Rook,
            'N' | 'n' => Kind::Knight,
            'B' | 'b' => Kind::Bishop,
            'Q' | 'q' => Kind::Queen,
            'K' | 'k' => Kind::King,
            c => Err(ProvidedIsNoValidPiece(c))?,
        })
    }
}
