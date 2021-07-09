trait Trait1 {
    fn foo(&self) -> i32 {
        1
    }
}

// derived trait
#[derive(Debug)]
struct Value {
    x: i32,
    y: i32,
}

impl Trait1 for Value {
    fn foo(&self) -> i32 {
        self.x + self.y
    }
}

// ____________________________________________________________

#[test]
fn trait_objects() {
    let value = Value { x: 10, y: 10 };
    let x: &dyn Trait1 = &value;
    assert_eq!(x.foo(), 20);
}

// ____________________________________________________________
// Super traits

trait Trait2 {}
trait Trait3 {}
trait Trait4: Trait2 + Trait3 {}

struct Structure;
impl Trait4 for Structure {}
impl Trait2 for Structure {}
impl Trait3 for Structure {}

// ____________________________________________________________
// Fully Qualified Syntax for Disambiguation

trait Foo {
    fn f(&self) -> String;
    fn foo_bar() -> String;
}

trait Bar {
    fn f(&self) -> String;
    fn foo_bar() -> String;
}

struct Baz;

impl Foo for Baz {
    fn f(&self) -> String {
        "Foo".to_string()
    }
    fn foo_bar() -> String {
        "Foo".to_string()
    }
}

impl Bar for Baz {
    fn f(&self) -> String {
        "Bar".to_string()
    }
    fn foo_bar() -> String {
        "Bar".to_string()
    }
}

impl Baz {
    fn f(&self) -> String {
        "Baz".to_string()
    }
    fn foo_bar() -> String {
        "Baz".to_string()
    }
}

#[test]
fn fully_qualified_syntax_for_disambiguation() {
    let value = Baz;
    assert_eq!(value.f(), "Baz");

    // Angle bracket form
    assert_eq!(<Baz as Foo>::f(&value), "Foo");
    assert_eq!(<Baz as Bar>::f(&value), "Bar");
    assert_eq!(<Baz>::f(&value), "Baz");

    assert_eq!(<Baz as Foo>::foo_bar(), "Foo");
    assert_eq!(<Baz as Bar>::foo_bar(), "Bar");
    assert_eq!(<Baz>::foo_bar(), "Baz");

    // Universal function call syntax
    assert_eq!(Foo::f(&value), "Foo");
    assert_eq!(Bar::f(&value), "Bar");
    assert_eq!(Baz::f(&value), "Baz");
}

// ____________________________________________________________
// Sealed traits

#[test]
fn sealed() {
    mod module {
        mod private {
            pub trait Sealed {}
        }

        /// This trait is sealed and cannot be implemented for types outside this crate.
        pub trait SealedTrait1: private::Sealed {
            fn foo(&self);
        }

        impl SealedTrait1 for usize {
            fn foo(&self) {}
        }
        impl private::Sealed for usize {}
    }

    let x: &dyn module::SealedTrait1 = &10;
    x.foo();
}

// ____________________________________________________________
// Conforming all types, which conform to a trait, to a trait

trait Hello {
    fn hello_string(&self) -> String;
}

impl<T: ToString> Hello for T {
    fn hello_string(&self) -> String {
        let mut hello = "Hello ".to_string();
        hello.push_str(&(self.to_string()));
        return hello;
    }
}

#[test]
fn conforming_all_types() {
    assert_eq!(10.hello_string(), "Hello 10");
}

//________________________________________________
// Impl dyn trait adds inherent methods to the trait object.

trait K {
    fn value(&self) -> i32;
}

impl dyn K {
    fn another_value(&self) -> i32 {
        self.value() + 1
    }
}

struct TupleStruct(i32);

impl K for TupleStruct {
    fn value(&self) -> i32 {
        self.0
    }
}

impl TupleStruct {
    fn another_value(&self) -> i32 {
        self.value() + 2
    }
}

#[test]
fn impl_dyn_trait() {
    let x: &dyn K = &TupleStruct(10);
    assert_eq!(x.another_value(), 11); // static binding
}

// ____________________________________________________________
// Impl trait is existential in an output position

#[test]
fn impl_trait_output() {
    fn foo<T: Copy + PartialEq + std::fmt::Debug>(x: T) -> impl Copy + PartialEq + std::fmt::Debug {
        x
    }

    let x1 = foo(1);
    let x2 = foo(1);
    assert_eq!(x1, x2);

    let y = foo('a');
    // assert_eq!(x1, y);

    fn foo1() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let x = foo1();
}

// ____________________________________________________________
// Impl trait is universal in an input position

#[test]
fn impl_trait_input() {
    //  syntax sugar for 'trait bound'
    fn foo(_x: impl Copy, y: impl Copy) {}
    foo(10, true);
}
