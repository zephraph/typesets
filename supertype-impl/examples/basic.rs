///
/// Example implementation:
///
/// enum Parent {
///   foo(String),
///   bar(String),
///   baz(uint8),
/// }
///
/// #[subset(Parent)]
/// enum Child {
///   foo(String)
/// }
///
/// impl TryFrom<Child> for Parent {
///   fn
/// }
///
/// impl TryFrom<String> for Child {
///   fn try_from(v: String) {
///     Net
///   }
/// }
///
///

fn main() {
    #[derive(Superset)]
    enum Parent {
        #[subset(Child, OtherChild)]
        Foo(String),
        #[subset(Child)]
        Bar(String),
        Baz(u8),
    }

    // Should produce
    // enum Parent {
    //     Foo(String),
    //     Bar(String),
    //     Baz(u8),
    // }

    // enum Child {
    //     Foo(String),
    //     Bar(String),
    // }

    // enum OtherChild {
    //     Foo(String),
    // }
}
