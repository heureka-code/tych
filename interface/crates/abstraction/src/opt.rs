use itertools::Itertools;
use pest::RuleType;

use crate::{CanBeSome, FromCppTypes, ParsingRulesDecl, ToCppTypes};

impl<R: RuleType + 'static, T: ToCppTypes<R>> ToCppTypes<R> for Option<T> {
    fn to_cpp_types(&self) -> String {
        if let Some(ref x) = self {
            format!("Some<{}>", x.to_cpp_types())
        } else {
            "None".into()
        }
    }
}

impl<T, R: RuleType + 'static> FromCppTypes<R> for Option<T>
where
    T: FromCppTypes<R> + CanBeSome<R>,
{
    fn _try_from_cpp(p: pest::iterators::Pair<'_, R>) -> Option<Self>
    where
        Self: Sized,
    {
        let rule = p.as_rule();
        Some(if rule == <T as CanBeSome<R>>::SOME_RULE {
            Some(T::try_from_cpp(p.into_inner().exactly_one().ok()?)?)
        } else if rule == <T as CanBeSome<R>>::NONE_RULE {
            None
        } else {
            None?
        })
    }
}

impl<T, R: RuleType + 'static> ParsingRulesDecl<R> for Option<T>
where
    T: CanBeSome<R>,
{
    fn _const_parses() -> impl Iterator<Item = R> {
        [T::NONE_RULE, T::SOME_RULE].into_iter()
    }
    fn _const_main_rule() -> R {
        T::OPTION_RULE
    }
}

#[macro_export]
macro_rules! impl_opt_rule {
    ($type: ident, $opt_rule: ident, $some_rule: ident, $none_rule: ident) => {
        impl abstraction::CanBeSome<crate::Rule> for $type {
            const OPTION_RULE: crate::Rule = crate::Rule::$opt_rule;
            const SOME_RULE: crate::Rule = crate::Rule::$some_rule;
            const NONE_RULE: crate::Rule = crate::Rule::$none_rule;
        }
    };

    ($type: ident) => {
        impl abstraction::CanBeSome<crate::Rule> for $type {
            const OPTION_RULE: crate::Rule = $crate::_deps::paste! { crate::Rule::[<Opt $type>] };
            const SOME_RULE: crate::Rule = $crate::_deps::paste! { crate::Rule::[<Some $type>] };
            const NONE_RULE: crate::Rule = crate::Rule::None;
        }
    };
}
