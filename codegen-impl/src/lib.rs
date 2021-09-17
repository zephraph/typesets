use proc_macro2::TokenStream;
use quote::quote;

/// This is our trivial code generation; one imagines we might want to do
/// something fancier, but this is sufficient to demonstrate how we might use
/// this collection of packages.
pub fn do_codegen() -> TokenStream {
    quote! {
        fn print_hi() {
            println!("I guess this is working!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::do_codegen;

    #[test]
    fn test() {
        assert_eq!(
            do_codegen().to_string(),
            r#"fn print_hi () { println ! ("I guess this is working!") ; }"#
        );
    }
}
