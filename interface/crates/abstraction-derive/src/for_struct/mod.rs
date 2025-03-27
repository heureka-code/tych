pub mod model;
pub(crate) mod parse_components;

use proc_macro::TokenStream;
use quote::{format_ident, quote};

use crate::components::StructComponentsVecExt;

pub(crate) fn from_cpp_types(name: syn::Ident, data: syn::DataStruct) -> impl Into<TokenStream> {
    let struct_data = model::Struct {
        name: name.clone(),
        components: iterate_struct_components(data.fields).collect(),
    };

    let parsed_data_iter_ident = format_ident!("parsed_data");

    let attribute_def_block = parse_components::wrapped_component_statements(
        &parsed_data_iter_ident,
        struct_data.components.iter().cloned(),
    );

    let struct_creation = struct_data.creation_as_quote();

    quote! {
        impl abstraction::FromCppTypes<crate::Rule> for #name {
            fn _try_from_cpp(p: pest::iterators::Pair<'_, crate::Rule>) -> Option<Self>
                where Self: Sized {

                let mut #parsed_data_iter_ident = p.into_inner();
                #attribute_def_block

                Some( #struct_creation )
            }
        }
    }
}

pub(crate) fn to_cpp_types(name: syn::Ident, data: syn::DataStruct) -> impl Into<TokenStream> {
    let struct_data = model::Struct {
        name: name.clone(),
        components: iterate_struct_components(data.fields).collect(),
    };

    let identifiers = struct_data
        .components
        .iter()
        .map(|c| c.attribute().field_access_expr());
    let f = struct_data.components.to_cpp_placeholder_format_str(&name);

    let (rust, cpp) = struct_data.to_comment_info();

    quote! {
        /// Constructs a C++ template type expression equivalent to an object of this Rust type
        ///
        #[doc = concat!(#rust, "\n")]
        ///
        #[doc = concat!("`", #cpp, "`\n")]
        impl abstraction::ToCppTypes<crate::Rule> for #name {
            fn to_cpp_types(&self) -> String
                where Self: Sized {

                format!( #f, #( #identifiers ),* )
            }
        }
    }
}

pub(crate) fn parsing_rules_decl(
    name: syn::Ident,
    _data: syn::DataStruct,
) -> impl Into<TokenStream> {
    let main_rule = &name;

    quote! {
        impl abstraction::ParsingRulesDecl<crate::Rule> for #name {
            fn _const_parses() -> impl Iterator<Item=crate::Rule> { std::iter::once( crate::Rule::#main_rule ) }
            fn _const_main_rule() -> crate::Rule { crate::Rule::#main_rule }
        }
    }
}

pub(crate) fn iterate_struct_components(
    fields: syn::Fields,
) -> impl Iterator<Item = model::StructComponent> {
    either::Either::Right(
        match fields {
            syn::Fields::Named(f) => f.named,
            syn::Fields::Unnamed(f) => f.unnamed,
            syn::Fields::Unit => return either::Either::Left(std::iter::empty()),
        }
        .into_iter()
        .enumerate()
        .map(|(index, field)| model::StructComponent {
            attribute: model::Attribute::new(index, field.ident),
            ty: field.ty,
        }),
    )
}
