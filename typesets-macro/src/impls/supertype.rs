use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Ident, ItemEnum, Variant};

use super::gen_enum::gen_subtype_enum;

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
            #[derive(Debug)]
            enum Sub1 {
                Variant1,
                Variant3 { x: u8, y: u8 }
            }

            impl std::convert::TryFrom<MyEnum> for Sub1 {
                type Error = crate::TypesetsError;

                fn try_from(supertype: MyEnum) -> Result<Self, Self::Error> {
                    match supertype {
                        MyEnum::Variant1 => Ok(Sub1::Variant1),
                        MyEnum::Variant3 { x, y } => Ok(Sub1::Variant3 { x, y }),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(MyEnum).to_string(),
                        subtype: stringify!(Sub1).to_string(),
                        variant: format!("{:?}", other).to_string(),
                    })

                    }
                }
            }

            impl From<Sub1> for MyEnum {
                fn from(child: Sub1) -> Self {
                    match child {
                        Sub1::Variant1 => MyEnum::Variant1,
                        Sub1::Variant3 { x, y } => MyEnum::Variant3 { x, y },
                    }
                }
            }

            #[derive(Debug)]
            enum Sub2 {
                Variant1,
                Variant2(u16, u8)
            }

            impl std::convert::TryFrom<MyEnum> for Sub2 {
                type Error = crate::TypesetsError;

                fn try_from(supertype: MyEnum) -> Result<Self, Self::Error> {
                    match supertype {
                        MyEnum::Variant1 => Ok(Sub2::Variant1),
                        MyEnum::Variant2(v0, v1) => Ok(Sub2::Variant2(v0, v1)),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(MyEnum).to_string(),
                        subtype: stringify!(Sub2).to_string(),
                        variant: format!("{:?}", other).to_string(),
                    })

                    }
                }
            }

            impl From<Sub2> for MyEnum {
                fn from(child: Sub2) -> Self {
                    match child {
                        Sub2::Variant1 => MyEnum::Variant1,
                        Sub2::Variant2(v0, v1) => MyEnum::Variant2(v0, v1)
                    }
                }
            }

            #[derive(Debug)]
            enum Sub3 {
                Variant2(u16, u8),
                Variant3 { x: u8, y: u8 }
            }

            impl std::convert::TryFrom<MyEnum> for Sub3 {
                type Error = crate::TypesetsError;

                fn try_from(supertype: MyEnum) -> Result<Self, Self::Error> {
                    match supertype {
                        MyEnum::Variant2(v0, v1) => Ok(Sub3::Variant2(v0, v1)),
                        MyEnum::Variant3 {x, y} => Ok(Sub3::Variant3 {x, y}),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(MyEnum).to_string(),
                        subtype: stringify!(Sub3).to_string(),
                        variant: format!("{:?}", other).to_string(),
                    })

                    }
                }
            }

            impl From<Sub3> for MyEnum {
                fn from(child: Sub3) -> Self {
                    match child {
                        Sub3::Variant2(v0, v1) => MyEnum::Variant2(v0, v1),
                        Sub3::Variant3 { x, y } => MyEnum::Variant3 { x, y },
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
