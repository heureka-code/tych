use abstraction::impl_ok_rule;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_more::Debug;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, FromCppTypes, ParsingRulesDecl, ToCppTypes,
)]
pub struct Unit {}

impl_ok_rule!(Unit, OkUnit);
