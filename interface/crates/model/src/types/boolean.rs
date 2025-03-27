use abstraction::{FromCppTypes, ParsingRulesDecl, ToCppTypes};

impl FromCppTypes<crate::Rule> for bool {
    fn _try_from_cpp(p: pest::iterators::Pair<'_, crate::Rule>) -> Option<Self>
    where
        Self: Sized,
    {
        match p.as_str() {
            "True" => Some(true),
            "False" => Some(false),
            _ => None,
        }
    }
}
impl ParsingRulesDecl<crate::Rule> for bool {
    fn _const_parses() -> impl Iterator<Item = crate::Rule> {
        [crate::Rule::BooleanTrue, crate::Rule::BooleanFalse].into_iter()
    }
    fn _const_main_rule() -> crate::Rule {
        crate::Rule::Boolean
    }
}

impl ToCppTypes<crate::Rule> for bool {
    fn to_cpp_types(&self) -> String {
        match self {
            true => "True",
            false => "False",
        }
        .to_string()
    }
}
