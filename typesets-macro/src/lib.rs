use proc_macro::TokenStream;
use typesets_impl::{subtype::gen_subtype, supertype::gen_supertype};

#[macro_use]
extern crate proc_macro_error;

/// This is a derive macro whose input is the item to which the `#[derive(..)]`
/// is applied.
#[proc_macro_derive(Supertype, attributes(subtype))]
#[proc_macro_error]
pub fn supertype_derive(item: TokenStream) -> TokenStream {
    gen_supertype(item.into()).into()
}

#[proc_macro_derive(Subtype, attributes(subtype_of))]
#[proc_macro_error]
pub fn subtype_of(input: TokenStream) -> TokenStream {
    gen_subtype(input.into()).into()
}
