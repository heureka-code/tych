use abstraction::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::Rule;

/// Equivalent to the variadic C++ template `Container`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Getters)]
pub struct Container<T> {
    items: Vec<T>,
}
impl<T> Container<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}

impl<T: ToCppTypes<crate::Rule>> ToCppTypes<crate::Rule> for Container<T> {
    fn to_cpp_types(&self) -> String {
        format!("Container<{}>", self.items().to_cpp_types())
    }
}

impl<T: FromCppTypes<crate::Rule>> abstraction::FromCppTypes<Rule> for Container<T>
where
    Container<T>: ParsingRulesDecl<crate::Rule>,
{
    fn _try_from_cpp(p: pest::iterators::Pair<'_, crate::Rule>) -> Option<Self> {
        let items = p
            .into_inner()
            .map(T::try_from_cpp)
            .collect::<Option<Vec<_>>>()?;
        Some(Self { items })
    }
}

impl<A> FromIterator<A> for Container<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<T> IntoIterator for Container<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
