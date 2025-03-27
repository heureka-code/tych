use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_more::Debug;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, FromCppTypes, ParsingRulesDecl, ToCppTypes,
)]
pub struct CastlingRight {
    kingside_allowed: bool,
    queenside_allowed: bool,
}
