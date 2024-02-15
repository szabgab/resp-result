use quote::quote;

use crate::ast_nodes::function_loader::Function;

mod arg_loader;
mod ast_nodes;
#[proc_macro_attribute]
pub fn resp_result(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = syn::parse_macro_input!(input as Function);
    quote!(#func).into()
}
