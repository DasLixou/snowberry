use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Attribute, Expr, Lit, Meta};

mod resource;

const SNOWBERRY_PATH: &str = "snowberry_path";
const INTERNAL: &str = "internal";

#[proc_macro_derive(Resource, attributes(snowberry_path))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    resource::derive_impl(input)
}

fn snowberry_path(attrs: &[Attribute], subcrate: &str) -> proc_macro2::TokenStream {
    let snowberry_path_attr = attrs
        .iter()
        .find(|attr| attr.path().is_ident(SNOWBERRY_PATH));
    if is_internal(snowberry_path_attr) {
        let ident = Ident::new(&format!("snowberry_{subcrate}"), Span::call_site());
        quote!(::#ident)
    } else {
        let ident = Ident::new(subcrate, Span::call_site());
        quote!(::snowberry::#ident)
    }
}

fn is_internal(attr: Option<&Attribute>) -> bool {
    let Some(attr) = attr else {
        return false;
    };
    let Meta::NameValue(nvmeta) = &attr.meta else {
        return false;
    };
    let Expr::Lit(lit) = &nvmeta.value else {
        return false;
    };
    let Lit::Str(str) = &lit.lit else {
        return false;
    };
    str.value().eq(INTERNAL)
}
