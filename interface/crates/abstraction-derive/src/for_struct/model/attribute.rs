use quote::{format_ident, quote};

use super::StructKind;

#[derive(Clone)]
pub struct Attribute {
    /// the position of the attribute in the definition list
    index: usize,
    /// only for non-tuple-structs, the name of the field to assign the value to
    field_name: Option<syn::Ident>,
    /// the name of the local variable that stores the parse result
    local_variable: syn::Ident,
}

impl Attribute {
    pub fn new_tuple(index: usize) -> Self {
        Attribute {
            index,
            field_name: None,
            local_variable: format_ident!("local_{index}"),
        }
    }
    pub fn new_mapping(index: usize, field_name: syn::Ident) -> Self {
        Attribute {
            index,
            local_variable: format_ident!("local_{field_name}"),
            field_name: Some(field_name),
        }
    }
    pub fn new(index: usize, field_name: Option<syn::Ident>) -> Self {
        if let Some(name) = field_name {
            Self::new_mapping(index, name)
        } else {
            Self::new_tuple(index)
        }
    }
    #[allow(unused)]
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn field_name(&self) -> Option<&syn::Ident> {
        self.field_name.as_ref()
    }
    pub fn field_access_expr(&self) -> proc_macro2::TokenStream {
        if let Some(n) = self.field_name() {
            quote! { self.#n.to_cpp_types() }
        } else {
            let index = syn::Index::from(self.index());
            quote! { self.#index.to_cpp_types() }
        }
    }
    pub fn local_variable(&self) -> &syn::Ident {
        &self.local_variable
    }

    pub fn is_tuple(&self) -> bool {
        self.field_name().is_none()
    }
    pub fn kind(&self) -> StructKind {
        if self.is_tuple() {
            StructKind::TupleFields
        } else {
            StructKind::NamedFields
        }
    }

    pub fn def_as_quote(&self) -> proc_macro2::TokenStream {
        let local = self.local_variable();
        if let Some(field) = self.field_name() {
            quote! { #field: #local }
        } else {
            quote! { #local }
        }
    }
}
