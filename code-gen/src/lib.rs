mod derive_resp_error;
mod proc_resp_result;
use quote::quote;

use proc_resp_result::Function;

/// convert a return [Result] [`Handler`](axum::Handler) return [`RespResult`]
#[proc_macro_attribute]
pub fn resp_result(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = syn::parse_macro_input!(input as Function);
    quote!(#func).into()
}
