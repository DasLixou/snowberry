use proc_macro::TokenStream;
use quote::quote;

use crate::snowberry_path;

pub fn derive_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let core = snowberry_path(&ast.attrs, "core");

    let name = &ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #core::resource::Resource for #name #ty_generics #where_clause {

        }
    }
    .into()
}
