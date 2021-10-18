// We pass through all the macro definitions so they can be used by consumers.
// If a package exports both macro and non-macro items, it's necessary to wrap
// the macro package like this as packages may directly export macros or
// regular items, but not both.
pub use codegen_macro::*;

// We don't export the code generation implementation, but rather wrap it here
// for use with a build.rs script.
use codegen_impl::do_codegen;

use rustfmt_wrapper::rustfmt;

/// This is what we'd expect build.rs scripts to call if they wanted to use our
/// code generator to create files rather than as macros. While one can see the
/// output from macros using `cargo expand` it's much easier if the code is
/// placed in a file. To make things even easier, we run the TokenStream
/// through rustfmt first.
pub fn build() -> String {
    let code = do_codegen();
    rustfmt(code).unwrap()
}
