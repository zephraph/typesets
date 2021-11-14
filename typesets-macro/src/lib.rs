use proc_macro::TokenStream;
use typesets_impl::supertype::gen_supertype;

/// This is a derive macro whose input is the item to which the `#[derive(..)]`
/// is applied.
#[proc_macro_derive(Supertype, attributes(subtype))]
pub fn supertype_derive(item: TokenStream) -> TokenStream {
    gen_supertype(item.into()).into()
}
