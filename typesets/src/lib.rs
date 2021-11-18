pub use typesets_macro::*;

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_supertype_derive() {
        #[derive(Supertype, Debug)]
        enum MyEnum {
            #[subtype(Sub1, Sub2)]
            Variant1,
            #[subtype(Sub2, Sub3)]
            Variant2(u16, u8),
            #[subtype(Sub3, Sub1)]
            Variant3 { x: u8, y: u8 },
            Variant4,
        }
        let e = MyEnum::Variant2(12, 0);
        let c: Sub2 = e.try_into().unwrap();
    }

}