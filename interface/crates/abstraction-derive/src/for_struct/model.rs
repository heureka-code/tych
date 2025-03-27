mod attribute;
mod component;
pub use attribute::Attribute;
pub use component::StructComponent;

use itertools::Itertools;
use quote::quote;

use crate::components::StructComponentsVecExt;

#[derive(Clone, Copy)]
pub enum StructKind {
    Unit,
    NamedFields,
    TupleFields,
}

#[derive(Clone)]
pub struct Struct {
    /// the name of the wrapper struct
    pub name: syn::Ident,
    // /// the rule variant associated with this struct
    // pub main_rule: Option<syn::ExprPath>,
    /// the fields of the struct, each of a type that abstracts a rule
    pub components: Vec<StructComponent>,
}

impl Struct {
    pub fn attr_creation_as_quote(
        struct_name: &proc_macro2::TokenStream,
        attributes: &Vec<Attribute>,
    ) -> proc_macro2::TokenStream {
        let qs = attributes.iter().map(|a| a.def_as_quote());
        if let Some(first) = attributes.first() {
            if first.is_tuple() {
                quote! { #struct_name( #(#qs),* ) }
            } else {
                quote! { #struct_name { #(#qs),* } }
            }
        } else {
            quote! { #struct_name { } }
        }
    }
    pub fn creation_as_quote(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        Self::attr_creation_as_quote(
            &quote! {#name},
            &self
                .components
                .iter()
                .map(|c| c.attribute().clone())
                .collect(),
        )
    }

    pub fn to_comment_info(&self) -> (String, String) {
        (
            self.components.to_rust_string(None, &self.name.to_string()),
            {
                match self.components.get_kind() {
                    StructKind::Unit => self.name.to_string(),
                    StructKind::NamedFields | StructKind::TupleFields => {
                        format!(
                            "{}<{}>",
                            self.name.to_string(),
                            self.components.field_identifier_strings().join(", ")
                        )
                    }
                }
            },
        )
    }
}
