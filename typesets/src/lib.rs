use thiserror::Error;
pub use typesets_macro::*;


#[derive(Error, Debug)]
pub enum TypesetsError {
    #[error("failed to convert type {subtype:?} which does not contain variant {variant:?} from {supertype:?}")]
    EnumNoOverlap {
        supertype: String,
        subtype: String,
        variant: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supertype_is_usable() {
        #[derive(Debug, Supertype)]
        enum MyEnum {
            #[subtype(Sub1)]
            Variant1,
            #[subtype(Sub1, Sub2)]
            Variant2(u8),
            #[subtype(Sub2)]
            Variant3 { x: u16, y: u16 }
        }
    }
}