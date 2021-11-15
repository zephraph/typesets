# Typesets

This library aims to provide a set of macros that help remove the boilerplate of expressing relationships between different types.

## Supertype

This derive macro provides the ability to generate enums which are a subset of the derived type.

To use, simply include the `Supertype` and denote which variants should go to which derived enums by the `subtype` attr.

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

The above will expand to...

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

## Subtype

Sometimes you'll want to describe that a given enum is a subtype of another enum without actually having access to the original enum.
This can be accomplished with the `subtype_of` attr macro.

```
#[subtype_of(SomeSuperType)]
enum MySubType {
  Variant1,
  Variant2(u8)
}
```

which will expand to

```rust
enum MySubType {
  Variant1,
  Variant2(u8)
}

impl TryFrom<SomeSuperType> for MySubType {
  type Error = crate::typesets::subtype::SubtypeError;

  fn try_from(parent: SomeSuperType) -> Result<Self, Self::Error> {
    match parent {
      SomeSuperType::Variant1 => MySubType::Variant1,
      SomeSuperType::Variant2(v0) => MySubType::Variant2(v0),
      other => Self::Error::EnumNoOverlap {
        supertype: "SomeSuperType",
        subtype: "MySubType",
        variant: format!("{:?}", other)
      }
    }
  }
}

impl From<MySubType> for SomeSuperType {
  fn for(child: MySubType) -> Self {
    match child {
      MySubType::Variant1 => MySupertype::Variant1,
      MySubType::Variant2(v0) => MySupertype::Variant2(v0),
    }
  }
}
```
