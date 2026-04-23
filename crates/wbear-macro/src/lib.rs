#![feature(proc_macro_expand)]

use proc_macro::TokenStream;
use quote::quote;

fn left_pad(quote: &str) -> String {
    quote
        .split("\n")
        .map(|s| format!("  {s}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[proc_macro]
pub fn include_quotelist(path: TokenStream) -> TokenStream {
    let data = path.expand_expr().unwrap();
    let data = syn::parse_macro_input!(data as syn::LitStr).value();
    let data: Vec<_> = data
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with("#"))
        .map(left_pad)
        .collect();

    quote! { [#(#data),*] }.into()
}
