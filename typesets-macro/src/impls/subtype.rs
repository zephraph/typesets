use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{ItemEnum, Variant};

use super::gen_enum::gen_enum_conversion;

pub fn gen_subtype(input: TokenStream) -> TokenStream {
    if let Ok(ItemEnum {
        variants, ident, attrs, ..
    }) = syn::parse2(input)
    {
        let supertype = attrs.iter().find(|a| a.path.is_ident("subtype_of"));
        let supertype: Ident = match supertype {
            Some(attr) => match attr.parse_args::<Ident>() {
                Ok(ident) => ident,
                Err(_) => abort_call_site!("Expected `subtype_of` attribute to take an ident argument"),
            },
            None => abort_call_site!("Expected `subtype_of` attribute describing what the super type is"),
        };
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
        let input = quote! {
          #[subtype_of(SomeSuperType)]
          enum MyEnum {
            Variant1,
            Variant2(u8),
            Variant3 { x: u16, y: u16 }
          }
        };

        let expected = quote! {
            impl std::convert::TryFrom<SomeSuperType> for MyEnum {
                type Error = crate::TypesetsError;

                fn try_from(supertype: SomeSuperType) -> Result<Self, Self::Error> {
                    match supertype {
                        SomeSuperType::Variant1 => Ok(MyEnum::Variant1),
                        SomeSuperType::Variant2(v0) => Ok(MyEnum::Variant2(v0)),
                        SomeSuperType::Variant3 {x, y} => Ok(MyEnum::Variant3 {x, y}),
                        other => Err(Self::Error::EnumNoOverlap {
                        supertype: stringify!(SomeSuperType).to_string(),
                        subtype: stringify!(MyEnum).to_string(),
                        variant: format!("{:?}", other).to_string(),
                    })

                    }
                }
            }

            impl From<MyEnum> for SomeSuperType {
                fn from(child: MyEnum) -> Self {
                    match child {
                        MyEnum::Variant1 => SomeSuperType::Variant1,
                        MyEnum::Variant2(v0) => SomeSuperType::Variant2(v0),
                        MyEnum::Variant3{x, y} => SomeSuperType::Variant3{x, y}
                    }
                }
            }
        };

        similar_asserts::assert_str_eq!(
            rustfmt(gen_subtype(input)).unwrap(),
            rustfmt(expected).unwrap()
        )
    }
}
