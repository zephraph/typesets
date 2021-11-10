use codegen_impl::do_codegen;
use proc_macro::TokenStream;

/// This is a derive macro whose input is the item to which the `#[derive(..)]`
/// is applied.
#[proc_macro_derive(Superset, attributes(subset))]
pub fn codegen_derive(item: TokenStream) -> TokenStream {
    do_codegen(item.into()).into()
}
