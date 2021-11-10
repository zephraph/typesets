# Supertype

This library provides a macro that strives to make it easier to derive a subset of a type from another type. This is useful when you want to centralize the total possible shape of a type and expose partials of that type for use later. Currently only enums are supported, but struct support may be added in the future.

Given this configuration

```rust
#[derive(Supertype)]
pub enum MyExpansiveType {
  #[subtype(MyNarrowerType, MyOtherType)]
  State1(String),
  #[subtype(MyNarrowerType)]
  State2(u8),
  #[subtype(MyOtherType)]
  State3,
  State4
}
```

You'll receive an output of

```rust
pub enum MyExpansiveType {
  State1(String),
  State2(u8),
  State3,
  State4
}

pub enum MyNarrowerType {
  State1(String),
  State2(u8)
}

pub enum MyOtherType {
  State1(String),
  State3,
}
```
