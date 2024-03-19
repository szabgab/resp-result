use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use crate::derive_resp_error::codegen::RespErrorCodeGen;
use crate::derive_resp_error::input::RespErrorDeriveInput;

mod input;
mod structure;
mod codegen;

pub fn gen_resp_error_derive(input:&DeriveInput)->syn::Result<TokenStream>{
    let input=RespErrorDeriveInput::from_derive_input(input)?;
    let codegen:RespErrorCodeGen = input.try_into()?;
    Ok(quote!(#codegen))
}