use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{ItemEnum, Variant};

use super::enum_helper::gen_enum_conversion;

pub fn gen_subtype(input: TokenStream, item: TokenStream) -> TokenStream {
    let supertype = syn::parse2::<Ident>(input).unwrap();
    if let Ok(ItemEnum {
        variants, ident, ..
    }) = syn::parse2(item)
    {
        let mut variants_list: Vec<Variant> = Vec::default();
        for variant in variants {
            variants_list.push(variant);
        }
        let conversion = gen_enum_conversion(&ident, &supertype, &variants_list);
        quote! { #conversion }
    } else {
        abort_call_site!("Subtype currently only supports enums")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustfmt_wrapper::rustfmt;

    #[test]
    fn test() {
        let attr = quote! { SomeSuperType };
        let input = quote! {
          enum MyEnum {
            Variant1,
            Variant2(u8),
            Variant3 { x: u16, y: u16 }
          }
        };

        let expected = quote! {
            impl std::convert::TryFrom<SomeSuperType> for MyEnum {
                type Error = crate::typesets_macro::gen::supertype::SupertypeError;

                fn try_from(supertype: SomeSuperType) -> Result<Self, Self::Error> {
                    match supertype {
                        SomeSuperType::Variant1 => Ok(MyEnum::Variant1),
                        SomeSuperType::Variant2(v0) => Ok(MyEnum::Variant2(v0)),
                        SomeSuperType::Variant3 {x, y} => Ok(MyEnum::Variant3 {x, y}),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(SomeSuperType),
                        subtype: stringify!(MyEnum),
                        variant: format!("{:?}", other)
                    })

                    }
                }
            }

            impl From<MyEnum> for SomeSuperType {
                fn from(child: MyEnum) -> Self {
                    match child {
                        MyEnum::Variant1 => SomeSuperType::Variant1,
                        MyEnum::Variant2(v0) => SomeSuperType::Variant2(v0),
                        MyEnum::Variant3{x, y} => SomeSuperType::Variant3{x, y},
                    }
                }
            }
        };

        similar_asserts::assert_str_eq!(
            rustfmt(gen_subtype(attr, input)).unwrap(),
            rustfmt(expected).unwrap()
        )
    }
}
