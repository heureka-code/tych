use std::str::FromStr;

use abstraction::{FromCppTypes, ParsingRulesDecl, ToCppTypes};

use super::{
    castling_move::{CastlingMove, ColoredCastlingMove},
    Container, NormalMove,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AnyMove {
    Normal(NormalMove),
    Castling(CastlingMove),
    ColoredCastling(ColoredCastlingMove),
}

#[derive(Debug, derive_more::Display, thiserror::Error)]
#[display("AnyMoveParsingError")]
pub struct AnyMoveParsingError;

impl FromStr for AnyMove {
    type Err = AnyMoveParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let castling = s
            .parse()
            .map(Self::Castling)
            .map_err(|_| AnyMoveParsingError);
        let normal = s.parse().map(Self::Normal).map_err(|_| AnyMoveParsingError);

        castling.or(normal)
    }
}

impl ToCppTypes<crate::Rule> for AnyMove {
    fn to_cpp_types(&self) -> String {
        match self {
            Self::Normal(n) => n.to_cpp_types(),
            Self::Castling(c) => c.to_cpp_types(),
            Self::ColoredCastling(cc) => cc.to_cpp_types(),
        }
    }
}
impl FromCppTypes<crate::Rule> for AnyMove {
    fn _try_from_cpp(p: pest::iterators::Pair<'_, crate::Rule>) -> Option<Self>
    where
        Self: Sized,
    {
        let normal = NormalMove::try_from_cpp(p.clone()).map(Self::Normal);
        let castling = CastlingMove::try_from_cpp(p.clone()).map(Self::Castling);
        let colored_castling = ColoredCastlingMove::try_from_cpp(p).map(Self::ColoredCastling);
        normal.or(castling).or(colored_castling)
    }
}
impl ParsingRulesDecl<crate::Rule> for AnyMove {
    fn _const_main_rule() -> crate::Rule {
        crate::Rule::AnyMove
    }
    fn _const_parses() -> impl Iterator<Item = crate::Rule> {
        std::iter::once(Self::_const_main_rule())
            .chain(NormalMove::_const_parses())
            .chain(CastlingMove::_const_parses())
            .chain(ColoredCastlingMove::_const_parses())
    }
}

pub type MoveContainer = Container<AnyMove>;
abstraction::impl_struct_main_rule!(MoveContainer);
abstraction::impl_opt_rule!(MoveContainer);
