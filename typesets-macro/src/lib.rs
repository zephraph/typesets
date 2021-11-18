mod gen;
use gen::*;

#[macro_use]
extern crate proc_macro_error;

use proc_macro::TokenStream;

/// This is a derive macro whose input is the item to which the `#[derive(..)]`
/// is applied.
#[proc_macro_derive(Supertype, attributes(subtype))]
#[proc_macro_error]
pub fn supertype_derive(item: TokenStream) -> TokenStream {
    supertype::gen_supertype(item.into()).into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn subtype_of(input: TokenStream, item: TokenStream) -> TokenStream {
    subtype::gen_subtype(input.into(), item.into()).into()
}