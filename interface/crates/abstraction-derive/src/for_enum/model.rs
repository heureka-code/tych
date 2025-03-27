use itertools::Itertools;
use quote::{format_ident, quote};

use crate::{
    components::StructComponentsVecExt,
    for_struct::{
        model::{Struct, StructComponent, StructKind},
        parse_components::wrapped_component_statements,
    },
};

pub struct EnumVariant {
    pub enum_name: syn::Ident,
    pub variant_name: syn::Ident,
    pub components: Vec<StructComponent>,
}

// pub struct Enum {
//     pub name: syn::Ident,
//     pub main_rule: Option<syn::ExprPath>,
//     pub variants: Vec<EnumVariant>,
// }

impl EnumVariant {
    pub fn rule_name(&self) -> syn::Ident {
        format_ident!("{}{}", self.enum_name, self.variant_name)
    }
    pub fn match_line_as_quote(&self) -> proc_macro2::TokenStream {
        // Rule::ColorWhite => Some(Color::White),
        let enum_name = &self.enum_name;
        let variant_name = &self.variant_name;
        let rule_name = self.rule_name();
        match self.get_kind() {
            StructKind::Unit => quote! {
                crate::Rule::#rule_name => Some(#enum_name::#variant_name),
            },
            StructKind::NamedFields | StructKind::TupleFields => {
                let parsed_data_iter_ident = format_ident!("parsed_data");

                let attribute_def_block = wrapped_component_statements(
                    &parsed_data_iter_ident,
                    self.components.iter().cloned(),
                );
                let ident = quote!(Self::#variant_name);
                let creation = Struct::attr_creation_as_quote(
                    &ident,
                    &self.components.attributes().cloned().collect(),
                );
                quote! {
                    crate::Rule::#rule_name => {
                        let mut #parsed_data_iter_ident = p.into_inner();
                        #attribute_def_block

                        Some( #creation )
                    }
                }
            }
        }
    }

    pub fn get_kind(&self) -> StructKind {
        self.components.get_kind()
    }

    pub fn to_comment_info(&self) -> (String, String) {
        (
            self.components.to_rust_string(
                Some(&self.enum_name.to_string()),
                &self.variant_name.to_string(),
            ),
            {
                match self.get_kind() {
                    StructKind::Unit => self.variant_name.to_string(),
                    StructKind::NamedFields | StructKind::TupleFields => {
                        format!(
                            "{}<{}>",
                            self.variant_name.to_string(),
                            self.components.field_identifier_strings().join(", ")
                        )
                    }
                }
            },
        )
    }

    pub fn to_cpp_match_line(&self) -> proc_macro2::TokenStream {
        let variant = &self.variant_name;
        let text = variant.to_string();

        let f = self
            .components
            .to_cpp_placeholder_format_str(&self.variant_name);

        match self.get_kind() {
            StructKind::Unit => quote! {
                Self::#variant => String::from(#text)
            },
            StructKind::TupleFields => {
                let locals: Vec<_> = self.components.local_variables().collect();
                quote! {
                    Self::#variant( #(#locals),* ) => format!(#f, #(#locals),*)
                }
            }
            StructKind::NamedFields => {
                let fields: Vec<_> = self.components.field_names().collect();
                quote! {
                    Self::#variant { #(#fields),* } => format!(#f, #(#fields),*)
                }
            }
        }
    }
    pub fn rule_path_as_quote(&self) -> proc_macro2::TokenStream {
        let rule_name = self.rule_name();
        quote! {
            crate::Rule::#rule_name
        }
    }
}
