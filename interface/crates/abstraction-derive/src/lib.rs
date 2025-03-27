mod components;
mod for_enum;
mod for_struct;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Implements `FromCppTypes` for the given type.
///
/// Structs get parsed by parsing the inner rules for the different struct components _in order_.
/// **The order they get declared in the struct matters!**
///
/// The trait requires `ParsingRulesDecl` to be implemented.
/// It will use the set main rule for parsing when no rule is provided and will ensure that the
/// rule of the main item is one of the allowed subrules
///
/// Enums will get parsed by parsing the variants like structs.
/// The main rule should be silent and be equivalent to the enum.
/// The variants are separate rules and these rules are included into the parsed output.
#[proc_macro_derive(FromCppTypes)]
pub fn from_cpp_types(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => for_struct::from_cpp_types(name, data).into(),
        syn::Data::Enum(data) => for_enum::from_cpp_types(name, data).into(),
        syn::Data::Union(_) => panic!("A union isn't useful for this derive"),
    };

    // Hand the output tokens back to the compiler
    expanded
}

/// Implements `ParsingRulesDecl` for the given type.
///
/// The definitions of this trait are used when the rules get deduced depending on the generic
/// target (Rust) type.
///
/// ## For structs
/// If the type is a struct, it's name is assumed to be a rule in the associated grammer
/// containing the attributes as sub rules (data)
///
/// The main rule of this type will be set to this and it will also be the only allowed sub rule
///
/// ## For enums
/// If the type is an enum, it's name is assumed to be a _silent_ rule that is a selection of different
/// rules. Each of these rules is of the form EnumNameVariantName, without spaces and with the same case.
///
/// The main rule of this type will be set to the rule with the same name as the enum variant.
/// The rules that are named based on the variants will be set as allowed sub rules.
#[proc_macro_derive(ParsingRulesDecl)]
pub fn parsing_rules_decl(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => for_struct::parsing_rules_decl(name, data).into(),
        syn::Data::Enum(data) => for_enum::parsing_rules_decl(name, data).into(),
        syn::Data::Union(_) => panic!("A union isn't useful for this derive"),
    };

    // Hand the output tokens back to the compiler
    expanded
}

/// Implements `ToCppTypes` for the given type.
///
/// This will generate a string representation of the Rust object as C++ template type expression
#[proc_macro_derive(ToCppTypes)]
pub fn to_cpp_types(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => for_struct::to_cpp_types(name, data).into(),
        syn::Data::Enum(data) => for_enum::to_cpp_types(name, data).into(),
        syn::Data::Union(_) => panic!("A union isn't useful for this derive"),
    };

    // Hand the output tokens back to the compiler
    expanded
}
