use proc_macro::TokenStream;

mod resource;

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    resource::derive_impl(input)
}
