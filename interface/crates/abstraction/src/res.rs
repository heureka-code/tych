use itertools::Itertools;
use pest::RuleType;

use crate::{CanBeErr, CanBeOk, FormsOkOfResult, FromCppTypes, ParsingRulesDecl, ToCppTypes};

impl<R: RuleType + 'static, T: ToCppTypes<R>, E: ToCppTypes<R>> ToCppTypes<R> for Result<T, E> {
    fn to_cpp_types(&self) -> String {
        match &self {
            Ok(t) => format!("Ok<{}>", t.to_cpp_types()),
            Err(e) => format!("Err<{}>", e.to_cpp_types()),
        }
    }
}

impl<T, E, R: RuleType + 'static> FromCppTypes<R> for Result<T, E>
where
    T: FromCppTypes<R> + CanBeOk<R> + FormsOkOfResult<R, E>,
    E: FromCppTypes<R> + CanBeErr<R>,
{
    fn _try_from_cpp(p: pest::iterators::Pair<'_, R>) -> Option<Self>
    where
        Self: Sized,
    {
        let rule = p.as_rule();
        Some(if rule == <T as CanBeOk<R>>::OK_RULE {
            Ok(T::try_from_cpp(p.into_inner().exactly_one().ok()?)?)
        } else if rule == <E as CanBeErr<R>>::ERR_RULE {
            Err(E::try_from_cpp(p.into_inner().exactly_one().ok()?)?)
        } else {
            None?
        })
    }
}

impl<T, E, R: RuleType + 'static> ParsingRulesDecl<R> for Result<T, E>
where
    T: CanBeOk<R> + FormsOkOfResult<R, E>,
    E: CanBeErr<R>,
{
    fn _const_parses() -> impl Iterator<Item = R> {
        [T::OK_RULE, E::ERR_RULE, T::RESULT_RULE].into_iter()
    }
    fn _const_main_rule() -> R {
        T::RESULT_RULE
    }
}

#[macro_export]
macro_rules! impl_ok_rule {
    ($type: ident, $ok_rule: ident) => {
        impl abstraction::CanBeOk<crate::Rule> for $type {
            const OK_RULE: crate::Rule = crate::Rule::$ok_rule;
        }
    };
}
#[macro_export]
macro_rules! impl_err_rule {
    ($type: ident, $err_rule: ident) => {
        impl abstraction::CanBeErr<crate::Rule> for $type {
            const ERR_RULE: crate::Rule = crate::Rule::$err_rule;
        }
    };
}
#[macro_export]
macro_rules! impl_res_rule {
    ($result_rule: ident, $ok: ident, $err: ident) => {
        impl abstraction::FormsOkOfResult<crate::Rule, $err> for $ok {
            const RESULT_RULE: crate::Rule = crate::Rule::$result_rule;
        }
    };
}
