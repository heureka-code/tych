use derive_more::{Debug, Display};

use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};

/// Either `White` or `Black`.
///
/// Typically represents the color of a player or a piece, but it can also be used to represent a
/// square's color.
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    FromCppTypes,
    ParsingRulesDecl,
    Display,
    Debug,
    ToCppTypes,
)]
pub enum Color {
    White,
    Black,
}
