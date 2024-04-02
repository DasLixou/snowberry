use proc_macro::TokenStream;
use quote::quote;

pub fn derive_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        // TODO: make this back to ::snowberry when we have cool macro internal find thingy
        impl #impl_generics snowberry::core::resource::Resource for #name #ty_generics #where_clause {

        }
    }
    .into()
}
