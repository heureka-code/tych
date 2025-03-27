#[macro_export]
macro_rules! impl_struct_main_rule {
    ($name: ty, $main_rule: ident) => {
        impl abstraction::ParsingRulesDecl<crate::Rule> for $name {
            fn _const_parses() -> impl Iterator<Item = crate::Rule> {
                std::iter::once(crate::Rule::$main_rule)
            }
            fn _const_main_rule() -> crate::Rule {
                crate::Rule::$main_rule
            }
        }
    };
    ($name: ident) => {
        impl abstraction::ParsingRulesDecl<crate::Rule> for $name {
            fn _const_parses() -> impl Iterator<Item = crate::Rule> {
                std::iter::once(crate::Rule::$name)
            }
            fn _const_main_rule() -> crate::Rule {
                crate::Rule::$name
            }
        }
    };
}
