use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Ident, ItemEnum, Variant};
use thiserror::Error;

// TODO
// 1. Variant arm to match arm
// 2. What will error type be for try from? std::error::Error

#[derive(Error, Debug)]
pub enum SupertypeError {
    #[error("failed to convert type {subtype:?} which does not contain variant {variant:?} from {supertype:?}")]
    EnumNoOverlap {
        supertype: &'static str,
        subtype: &'static str,
        variant: &'static str,
    },
}

fn variant_to_arm_partial(variant: &Variant) -> TokenStream {
    let name = variant.ident.clone();
    match variant.fields.clone() {
        syn::Fields::Named(fields) => {
            let field_idents: Vec<Ident> = fields
                .named
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .collect();
            quote! { #name { #(#field_idents),* } }
        }
        syn::Fields::Unnamed(fields) => {
            let field_idents: Vec<Ident> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, _)| Ident::new(format!("v{}", idx).as_str(), Span::call_site()))
                .collect();
            quote! { #name( #(#field_idents),* ) }
        }
        syn::Fields::Unit => quote! { #name },
    }
}

fn gen_subtype_enum(ident: Ident, parent: Ident, variants: Vec<Variant>) -> TokenStream {
    let arm_parts: Vec<TokenStream> = variants.iter().map(|v| variant_to_arm_partial(v)).collect();
    quote! {
        enum #ident {
            #(#variants),*
        }

        impl TryFrom<#parent> for #ident {
            type Error = crate::typesets::supertype::SupertypeError;
            fn try_from(parent: #parent) -> Result<Self, Self::Error> {
                match parent {
                    #(#parent::#arm_parts => Ok(#ident::#arm_parts)),*,
                    other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(#parent),
                        subtype: stringify!(#ident),
                        variant: format!("{:?}", other)
                    })
                }
            }
        }

        impl From<#ident> for #parent {
            fn from(child: #ident) -> Self {
                match child {
                    #(#parent::#arm_parts => #ident::#arm_parts),*
                }
            }
        }
    }
}

/// This is our trivial code generation; one imagines we might want to do
/// something fancier, but this is sufficient to demonstrate how we might use
/// this collection of packages.
pub fn gen_supertype(item: TokenStream) -> TokenStream {
    let mut child_to_variant: BTreeMap<Ident, Vec<Variant>> = BTreeMap::new();

    let parent_ident = if let Ok(ItemEnum {
        variants, ident, ..
    }) = syn::parse2(item)
    {
        for variant in variants {
            for (idx, attr) in variant.attrs.iter().enumerate() {
                if !attr.path.is_ident("subtype") {
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
        abort_call_site!("Supertype currently only supports enums")
    };

    let enums: Vec<TokenStream> = child_to_variant
        .into_iter()
        .map(|(ident, variants)| gen_subtype_enum(ident, parent_ident.clone(), variants))
        .collect();

    quote! {
        #(#enums)*

    }
}

#[cfg(test)]
mod tests {
    use rustfmt_wrapper::rustfmt;

    use super::*;
    use crate::supertype::gen_supertype;

    #[test]
    fn test() {
        let input = quote! {
            enum MyEnum {
                #[subtype(Sub1, Sub2)]
                Variant1,
                #[subtype(Sub2, Sub3)]
                Variant2(u16, u8),
                #[subtype(Sub3, Sub1)]
                Variant3 { x: u8, y: u8 },
                Variant4,
            }
        };

        let expected = quote! {
            enum Sub1 {
                Variant1,
                Variant3 { x: u8, y: u8 }
            }

            impl TryFrom<MyEnum> for Sub1 {
                type Error = crate::typesets::supertype::SupertypeError;

                fn try_from(parent: MyEnum) -> Result<Self, Self::Error> {
                    match parent {
                        MyEnum::Variant1 => Ok(Sub1::Variant1),
                        MyEnum::Variant3 { x, y } => Ok(Sub1::Variant3 { x, y }),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(MyEnum),
                        subtype: stringify!(Sub1),
                        variant: format!("{:?}", other)
                    })

                    }
                }
            }

            impl From<Sub1> for MyEnum {
                fn from(child: Sub1) -> Self {
                    match child {
                        MyEnum::Variant1 => Sub1::Variant1,
                        MyEnum::Variant3 { x, y } => Sub1::Variant3 { x, y },
                    }
                }
            }

            enum Sub2 {
                Variant1,
                Variant2(u16, u8)
            }

            impl TryFrom<MyEnum> for Sub2 {
                type Error = crate::typesets::supertype::SupertypeError;

                fn try_from(parent: MyEnum) -> Result<Self, Self::Error> {
                    match parent {
                        MyEnum::Variant1 => Ok(Sub2::Variant1),
                        MyEnum::Variant2(v0, v1) => Ok(Sub2::Variant2(v0, v1)),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(MyEnum),
                        subtype: stringify!(Sub2),
                        variant: format!("{:?}", other)
                    })

                    }
                }
            }

            impl From<Sub2> for MyEnum {
                fn from(child: Sub2) -> Self {
                    match child {
                        MyEnum::Variant1 => Sub2::Variant1,
                        MyEnum::Variant2(v0, v1) => Sub2::Variant2(v0, v1),
                    }
                }
            }

            enum Sub3 {
                Variant2(u16, u8),
                Variant3 { x: u8, y: u8 }
            }

            impl TryFrom<MyEnum> for Sub3 {
                type Error = crate::typesets::supertype::SupertypeError;

                fn try_from(parent: MyEnum) -> Result<Self, Self::Error> {
                    match parent {
                        MyEnum::Variant2(v0, v1) => Ok(Sub3::Variant2(v0, v1)),
                        MyEnum::Variant3 {x, y} => Ok(Sub3::Variant3 {x, y}),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(MyEnum),
                        subtype: stringify!(Sub3),
                        variant: format!("{:?}", other)
                    })

                    }
                }
            }

            impl From<Sub3> for MyEnum {
                fn from(child: Sub3) -> Self {
                    match child {
                        MyEnum::Variant2(v0, v1) => Sub3::Variant2(v0, v1),
                        MyEnum::Variant3 { x, y } => Sub3::Variant3 { x, y },
                    }
                }
            }
        };

        similar_asserts::assert_str_eq!(
            rustfmt(gen_supertype(input)).unwrap(),
            rustfmt(expected).unwrap()
        );
    }
}
