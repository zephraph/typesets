# Typesets

This library aims to provide a set of macros that help remove the boilerplate of expressing relationships between different types.

## Supertype

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

It'll expand to

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

impl TryFrom<MyExpansiveType> for MyNarrowerType {
  type Error = crate::typesets::supertype::SupertypeError;
  fn try_from(parent: MyExpansiveType) -> Result<Self, Self::Error> {
    match parent {
      MyExpansiveType::State1(v0) => Ok(MyNarrowerType::State1(v0)),
      MyExpansiveType::State2(v0) => Ok(MyNarrowerType::State2(v0)),
      other => Err(Self::Error::EnumNoOverlap {
        supertype: "MyExpansiveType",
        subtype: "MyNarrowerType",
        variant: format!("{:?}", other)
      })
    }
  }
}

impl From<MyNarrowerType> for MyExpansiveType {
  fn from(child: MyNarrowerType) -> Self {
    match child {
      MyNarrowerType::State1(v0) => MyExpansiveType::State1(v0),
      MyNarrowerType::State2(v0) => MyExpansiveType::State2(v0),
    }
  }
}

pub enum MyOtherType {
  State1(String),
  State3,
}


impl TryFrom<MyExpansiveType> for MyOtherType {
  type Error = crate::typesets::supertype::SupertypeError;
  fn try_from(parent: MyExpansiveType) -> Result<Self, Self::Error> {
    match parent {
      MyExpansiveType::State1(v0) => Ok(MyOtherType::State1(v0)),
      MyExpansiveType::State2(v0) => Ok(MyOtherType::State2(v0)),
      other => Err(Self::Error::EnumNoOverlap {
        supertype: "MyExpansiveType",
        subtype: "MyOtherType",
        variant: format!("{:?}", other)
      })
    }
  }
}

impl From<MyOtherType> for MyExpansiveType {
  fn from(child: MyOtherType) -> Self {
    match child {
      MyOtherType::State1(v0) => MyExpansiveType::State1(v0),
      MyOtherType::State2(v0) => MyExpansiveType::State2(v0),
    }
  }
}
```
