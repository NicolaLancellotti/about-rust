// You can now apply the #[used] attribute to static items to prevent the compiler
// from optimising them away, even if they appear to be unused,
#[used]
static FOO: u32 = 1;

#[test]
fn declarations_and_assignments() {
    // Constants
    const MAX_POINTS_CONST: i32 = 10_000; // #define MAX_POINTS_STATIC 10000
    static MAX_POINTS_STATIC: i32 = 10_000; // const int32_t MAX_POINTS_STATIC = 10000;

    // Declarations
    let const_x: i32;
    let const_ref_x: &i32;
    let const_mut_ref_x: &mut i32;

    let mut mut_x: i32;
    let mut mut_ref_x: &i32;
    let mut mut_mut_ref_x: &mut i32;

    // Assignments
    let float = 1e+12f64;
    let binary = 0x11;
    let octal = 0o11;
    let hexadecimal = 0x11;

    let ref_x = &10;
    let ref ref_x = 10;

    let mut_ref_x = &mut 10;
    let ref mut mut_ref_x = 10;

    // Assignments evaluate to ()
    let x;
    if (x = 2) == () {
        assert_eq!(x, 2);
    }

    // Blocks are expressions
    let (x, y) = {
        println!("Hello, world");
        (1, 2)
    };
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    // A const block is a variant of a block expression whose
    // body evaluates at compile-time instead of at runtime.
    let x = const { 1 + 2 };
    assert_eq!(x, 3);
}

#[test]
fn tuples_types() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    //  Destructure a tuple
    let (x, y, z) = tuple;
    let x = tuple.0;
    let one_element_tuple = (1,);
}

#[test]
fn tuple_structs_types() {
    struct TupleStruct(i32, i32, i32);
    let tuple_struct = TupleStruct(1, 2, 3);
    assert_eq!(tuple_struct.0, 1);
}

#[test]
fn unit_types() {
    // ZSTs Zero Sized Types
    struct Unit1;
    let unit = Unit1;
}

#[test]
fn empty_types() {
    enum Void {}
}

#[test]
fn enum_types() {
    enum Enum {
        UnnamedFields(u8, u8, u8),
        NamedFields { x: i32, y: i32 },
    }

    impl Enum {
        fn value(&self) -> i32 {
            match self {
                Self::UnnamedFields(_, _, _) => 0,
                Self::NamedFields { x: _, y: _ } => 1,
            }
        }
    }

    let enum1 = Enum::UnnamedFields(127, 0, 0);
    let enum2 = Enum::NamedFields { x: 1, y: 1 };

    // Matches Are Exhaustive
    match enum2 {
        Enum::UnnamedFields(a, 2, _) if a > 0 => {}
        _ => {}
    }

    if let Enum::UnnamedFields(_a, 2, _) = enum2 {}

    enum CLike {
        Variant1,
        Variant2,
    }
    assert_eq!(CLike::Variant1 as i32, 0);
}

#[test]
fn struct_types() {
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        // fn consume(self: Self) {}
        fn consume(self) {}

        // fn area(self: &Self)
        fn area(&self) -> u32 {
            self.length * self.width
        }

        // fn increment(self: &mut Self)
        fn increment(&mut self) {
            self.length += 1;
            self.width += 1;
        }

        fn can_contain(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    impl Rectangle {
        // Associated functions
        fn new(length: u32, width: u32) -> Rectangle {
            Rectangle {
                length: length,
                width: width,
            }

            // field init shorthand.
            // Rectangle {
            //     length,
            //     width,
            // }
        }

        fn new_with_same_width(length: u32, other: &Rectangle) -> Self {
            Rectangle { length, ..*other }
        }

        fn box_method(self: Box<Self>) {}
    }

    let rect = Rectangle::new(10, 15);
    assert_eq!(rect.area(), 150);

    //  Destructure a struct
    let Rectangle { length, width } = rect;
    assert_eq!(length, 10);
    assert_eq!(width, 15);

    let box_rect = Box::new(rect);
    box_rect.box_method();
    // rect.box_method(); // Error
}

#[test]
fn never_type() {
    fn bar() -> ! {
        loop {}
    }
}

// Alias
type Kilometers = i32;

#[test]
fn alignment() {
    #[repr(align(16))]
    struct Align16(i32);

    assert_eq!(std::mem::align_of::<Align16>(), 16);
    assert_eq!(std::mem::size_of::<Align16>(), 16);
}

#[test]
fn std_library_types() {
    // Array
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let array2: [i32; 10] = [0; 10];
    let first = array1[0];
    let first = &array1[0];
    let first = array1.get(1);

    // Vector
    let vector: Vec<i32> = Vec::new();
    let mut vector = vec![1, 2, 3];
    vector.push(5);

    // String
    let string = "Hello, world!".to_string();
    let mut string = String::new();
    string.push_str("Hello, world!");
    let string = String::from("Hello, world!");

    let chars = "नमस्ते".chars();

    let c_string = c"abc";
    let byte_string = b"asd";
    let raw_string = r#"a"sd"#;
    let multiline_string = "foo\
bar
a";
    assert_eq!("foobar\na", multiline_string);

    // Hash map
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    let x = map.entry("key2".to_string()).or_insert(2);

    let keys = vec!["key1".to_string(), "key2".to_string()];
    let values = vec![10, 20];
    let map: HashMap<_, _> = keys.into_iter().zip(values).collect();
    let map = map;
    let value = map.get(&"a".to_string());
    for (key, value) in &map {}

    // Ranges
    let one_to_two = 1..=2;
    let one_to_two = 1..3;
}
