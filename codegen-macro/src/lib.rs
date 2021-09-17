use codegen_impl::do_codegen;
use proc_macro::TokenStream;

/// This is a function-like macro that stands alone. Its item is the stream of
/// tokens within the delimiters of the macro invocation.
#[proc_macro]
pub fn codegen_macro(_item: TokenStream) -> TokenStream {
    do_codegen().into()
}

/// This is a derive macro whose input is the item to which the `#[derive(..)]`
/// is applied.
#[proc_macro_derive(Codegen)]
pub fn codegen_derive(_item: TokenStream) -> TokenStream {
    do_codegen().into()
}

/// This is an attribute macro where `attr` is the parameters to the macro and
/// `item` is the item to which the attribute is applied.
#[proc_macro_attribute]
pub fn codegen_attr(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    item.extend(TokenStream::from(do_codegen()));
    item
}
