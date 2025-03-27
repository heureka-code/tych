use quote::quote;

use super::model::StructComponent;

/// creates the statements that consume the data iterator and parse each element with the matching type
pub(crate) fn wrapped_component_statements(
    data_iter: &syn::Ident,
    components: impl Iterator<Item = StructComponent>,
) -> proc_macro2::TokenStream {
    let definitions = components.map(
        |comp| {
            let local = comp.attribute.local_variable();
            let ty = comp.ty();

            quote! {
                let #local = <#ty as abstraction::FromCppTypes<crate::Rule>>::try_from_cpp(#data_iter.next()?)?;
            }
        }
    );

    quote! { #( #definitions )* }.into()
}
