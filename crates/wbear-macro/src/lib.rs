use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn my_macro(_: TokenStream) -> TokenStream {
    quote! { "Hello, world!"; }.into()
}
