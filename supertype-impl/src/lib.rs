use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemEnum, Variant};

// TODO
// 1. Variant arm to match arm
// 2. What will error type be for try from? std::error::Error


fn generate_enum(ident: Ident, parent: Ident, variants: Vec<Variant>) -> TokenStream {
    quote! {
        enum #ident {
            #(#variants),*
        }

        impl TryFrom<#parent> for #ident {
            let Error = String;
            fn try_from(parent: #parent) -> Result<Self, Self::Err> {
                match parent {
                    #(#parent::variants => #ident::variants),*
                }
            }
        }

        impl From<#ident> for #parent {
            fn from() {

            }
        }
    }
}

/// This is our trivial code generation; one imagines we might want to do
/// something fancier, but this is sufficient to demonstrate how we might use
/// this collection of packages.
pub fn do_codegen(item: TokenStream) -> TokenStream {
    let mut child_to_variant: BTreeMap<Ident, Vec<Variant>> = BTreeMap::new();

    let parent_ident = if let Ok(ItemEnum { variants, ident, .. }) = syn::parse2(item) {
        for variant in variants {
            for (idx, attr) in variant.attrs.iter().enumerate() {
                if !attr.path.is_ident("subset") {
                    continue;
                }
                let meta = attr.parse_meta().unwrap();
                let list = match meta {
                    syn::Meta::List(list) => list,
                    _ => panic!(),
                };

                let mut variant = variant.clone();
                variant.attrs.remove(idx);

                for item in list.nested {
                    let ident = match item {
                        syn::NestedMeta::Meta(meta) => meta.path().get_ident().unwrap().clone(),
                        _ => panic!(),
                    };
                    //child_to_variant.insert(ident, )

                    child_to_variant
                        .entry(ident)
                        .or_default()
                        .push(variant.clone());
                }
            }
        }

        ident
    } else {
        panic!() // not an enum
    }

    let enums: Vec<TokenStream> = child_to_variant
        .into_iter()
        .map(|(ident, variants)| generate_enum(ident, parent_ident, variants))
        .collect();

    quote! {
        #(#enums)*

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::do_codegen;

    #[test]
    fn test() {
        let input = quote! {
            enum MyEnum {
                #[subset(Super1, Super2)]
                Variant1(u16),
                #[subset(Super2, Super3)]
                Variant2,
                #[subset(Super3, Super1)]
                Variant3,
                Variant4,
            }
        };

        let expected = quote! {
            enum Super1 {
                Variant1(u16),
                Variant3
            }

            impl TryFrom<MyEnum> for Super1 {
                type Error = ;

                fn try_from(value: MyEnum) -> Result<Self, Self::Error> {
                    match value {
                        Variant1(v1) => Variant1(v1),
                        Variant3 => Variant3,
                        _ => 
                    }
                }
            }

            enum Super2 {
                Variant1(u16),
                Variant2
            }

            enum Super3 {
                Variant2,
                Variant3
            }
        };

        assert_eq!(do_codegen(input).to_string(), expected.to_string());
    }
}
