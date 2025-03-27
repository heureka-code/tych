use itertools::Itertools;
use quote::ToTokens;

use crate::for_struct::model::{Attribute, StructComponent, StructKind};

pub trait StructComponentsVecExt {
    fn get_kind(&self) -> StructKind;
    fn same_count_placeholders(&self) -> impl Iterator<Item = proc_macro2::TokenStream>;
    fn field_names(&self) -> impl Iterator<Item = &syn::Ident>;
    fn local_variables(&self) -> impl Iterator<Item = &syn::Ident>;
    fn to_cpp_placeholder_format_str(&self, name: &syn::Ident) -> String;
    fn attributes(&self) -> impl Iterator<Item = &Attribute>;
    fn to_rust_string(&self, enum_name: Option<&str>, name: &str) -> String;
    fn field_identifier_strings(&self) -> impl Iterator<Item = String>;
}

impl StructComponentsVecExt for Vec<StructComponent> {
    fn get_kind(&self) -> StructKind {
        self.first()
            .map(|f| f.attribute().kind())
            .unwrap_or(StructKind::Unit)
    }

    fn same_count_placeholders(&self) -> impl Iterator<Item = proc_macro2::TokenStream> {
        self.iter().map(|_| {
            quote::quote! { {} }
        })
    }
    fn field_names(&self) -> impl Iterator<Item = &syn::Ident> {
        self.iter().flat_map(|f| f.attribute().field_name())
    }
    fn field_identifier_strings(&self) -> impl Iterator<Item = String> {
        self.iter().map(|f| {
            f.attribute()
                .field_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| f.attribute().index().to_string())
        })
    }
    fn local_variables(&self) -> impl Iterator<Item = &syn::Ident> {
        self.iter().map(|f| f.attribute().local_variable())
    }
    fn to_cpp_placeholder_format_str(&self, name: &syn::Ident) -> String {
        let placeholders = self.same_count_placeholders();
        if self.len() > 0 {
            quote::quote! {
                #name<#(#placeholders), *>
            }
        } else {
            quote::quote! { #name }
        }
        .to_string()
    }
    fn to_rust_string(&self, enum_name: Option<&str>, name: &str) -> String {
        let fields = self
            .iter()
            .map(|field| match field.attribute.kind() {
                StructKind::TupleFields => field.ty().to_token_stream().to_string(),
                StructKind::NamedFields => field
                    .attribute()
                    .field_name()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                StructKind::Unit => "".to_string(),
            })
            .join(", ");
        // format!("[self::{}#variant.{}]", self.enum_name.to_string(), self.variant_name.to_string())
        if let Some(enum_name) = enum_name {
            match self.get_kind() {
                StructKind::Unit => format!("[`{1}`][self::{0}#variant.{1}]", enum_name, name),
                StructKind::TupleFields => {
                    format!("[`{name}({})`][self::{enum_name}#variant.{name}]", fields)
                }
                StructKind::NamedFields => format!(
                    "[`{name}{{ {} }}`][self::{enum_name}#variant.{name}]",
                    fields
                ),
            }
        } else {
            match self.get_kind() {
                StructKind::Unit => format!("[`{name}`][{name}]"),
                StructKind::TupleFields => format!("[`{name}({})`][{name}]", fields),
                StructKind::NamedFields => format!("[`{name}{{ {} }}`][{name}]", fields),
            }
        }
    }
    fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.iter().map(|c| c.attribute())
    }
}
