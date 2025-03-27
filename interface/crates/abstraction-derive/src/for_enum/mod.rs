use proc_macro::TokenStream;
use quote::quote;

use crate::for_struct::iterate_struct_components;

pub mod model;

pub(crate) fn from_cpp_types(name: syn::Ident, data: syn::DataEnum) -> impl Into<TokenStream> {
    let match_lines =
        iterate_enum_variants(name.clone(), data.variants).map(|v| v.match_line_as_quote());

    quote! {
        impl abstraction::FromCppTypes<crate::Rule> for #name {
            fn _try_from_cpp(p: pest::iterators::Pair<'_, crate::Rule>) -> Option<Self>
                where Self: Sized {

                match p.as_rule() {
                    #( #match_lines )*
                    _ => None,
                }
            }
        }
    }
}

pub(crate) fn to_cpp_types(name: syn::Ident, data: syn::DataEnum) -> impl Into<TokenStream> {
    let output_matches =
        iterate_enum_variants(name.clone(), data.variants.clone()).map(|o| o.to_cpp_match_line());

    let info = iterate_enum_variants(name.clone(), data.variants).map(|o| {
        let (r, c) = o.to_comment_info();
        format!("{r} | `{c}`")
    });

    quote! {
        /// Constructs a C++ template type expression equivalent to an object of this Rust type
        ///
        /// | Rust  |  C++ |
        /// | ----- | ---- |
        #(#[doc = concat!("| ", #info, " | \n")])*
        impl abstraction::ToCppTypes<crate::Rule> for #name {
            fn to_cpp_types(&self) -> String
                where Self: Sized {

                match self {
                    #( #output_matches, )*
                }
            }
        }
    }
}

pub(crate) fn parsing_rules_decl(name: syn::Ident, data: syn::DataEnum) -> impl Into<TokenStream> {
    let rule_names =
        iterate_enum_variants(name.clone(), data.variants).map(|v| v.rule_path_as_quote());
    let main_rule = &name;

    quote! {
        impl abstraction::ParsingRulesDecl<crate::Rule> for #name {
            fn _const_parses() -> impl Iterator<Item=crate::Rule> {
                [ #(#rule_names),* ].into_iter()
            }
            fn _const_main_rule() -> crate::Rule {
                crate::Rule::#main_rule
            }
        }
    }
}

fn iterate_enum_variants(
    name: syn::Ident,
    variants: impl IntoIterator<Item = syn::Variant>,
) -> impl Iterator<Item = model::EnumVariant> {
    variants.into_iter().map(move |variant| model::EnumVariant {
        enum_name: name.clone(),
        variant_name: variant.ident,
        components: iterate_struct_components(variant.fields).collect(),
    })
}
