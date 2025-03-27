use super::Attribute;
// use quote::quote;

#[derive(Clone)]
pub struct StructComponent {
    pub attribute: Attribute,
    pub ty: syn::Type,
}

impl StructComponent {
    pub fn attribute(&self) -> &Attribute {
        &self.attribute
    }
    pub fn ty(&self) -> &syn::Type {
        &self.ty
    }

    /*pub fn def_as_quote(&self) -> proc_macro2::TokenStream {
        let ty = self.ty();
        if let Some(field) = self.attribute.field_name() {
            quote! { #field: #ty }
        } else {
            quote! { #ty }
        }
    }*/
}
