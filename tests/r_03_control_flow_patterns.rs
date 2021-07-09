#[test]
fn control_flow() {
    let value = if true {
        5
    } else if false {
        4
    } else {
        1
    };
    assert_eq!(value, 5);

    let result = loop {
        break 4;
    };

    'label: while value < 0 {
        break 'label;
    }

    let mut vec = vec![10, 20, 30, 40, 50];
    for element in vec.iter() {}
    for element in vec.iter_mut() {}
    for element in vec.into_iter() {}

    // Early return
    let value = match Some(5) {
        Some(x) => x,
        None => {
            assert!(false);
            return;
        }
    };
    assert_eq!(value, 5);

    // let-else statement
    let Some(value) = Some(5) else {
        assert!(false);
        return;
    };
    assert_eq!(value, 5);

    // break from labeled blocks
    let value = 'block: {
        if true {
            break 'block 5;
        }
        10
    };
    assert_eq!(value, 5);
}

#[test]
fn patterns() {
    // ________________________________________________
    // Irrefutable patterns

    {
        // let statement
        let (x, y, z) = (1, 2, 3);
    }

    {
        // function parameters
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({x}, {y})");
        }

        let point = (3, 5);
        print_coordinates(&point);
    }

    {
        fn foo(ref mut x: i32) {
            *x = 20;
        }
        let x = 10;
        foo(x);
        assert_eq!(x, 10);
    }

    // ____________________________
    // Destructuring
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point { x: 0, y: 7, z: 1 };
    let Point { x, y, z } = p;
    let Point { x: a, y: b, z: c } = p;

    match p {
        Point { x, y: 0, z } => println!("On the x axis at {x} {z}"),
        Point { x: 0, .. } => println!("x is {x}"),
        Point { x, y, z } => println!("On neither axis: ({x}, {y} {z})"),
    }

    match &p {
        &Point { x, y: 0, z } => println!("On the x axis at {x} {z}"),
        _ => println!("On the x axis at {x}"),
    }

    // Destructuring assignments

    let (a, b) = (1, 2);
    assert_eq!([1, 2], [a, b]);

    let [a, .., b, _] = [10, 20, 30, 40, 50];
    assert_eq!([10, 40], [a, b]);

    let Point { x: e, .. } = Point { x: 5, y: 3, z: 1 };
    assert_eq!(e, 5);

    // ________________________________________________
    // Refutable patterns

    {
        // if let
        let k = Some(10);
        if let Some(k) = k {}
    }

    {
        // While let
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);

        while let Some(_) = stack.pop() {}
    }

    {
        match Some(100) {
            Some(0) => println!("0"),                          // literal
            Some(1 | 2) => println!("1 or 2"),                 // multiple patterns
            Some(3..=4) => println!("3 or 4"),                 // ranges
            Some(id @ 5..=7) => println!("{id}"),              // bindings
            Some(x) if x >= 8 && x <= 9 => println!("8 or 9"), // named variable
            Some(10..) => println!(">= 10"),                   // open range pattern
            Some(_) => println!("anything"),                   // ignoring value
            _ => (),
        }

        match (10, 11) {
            (10, x) | (x, 10) => println!("{x}"),
            _ => (),
        }

        // Subslice patterns
        let _ = match [1, 2, 3] {
            [1, ..] => "starts with one",
            [.., 1] => "ends with one",
            rest => "something else",
        };

        //  automatically reference or de-reference in match statements.
        let arg = &Some("test".to_string());
        match arg {
            &Some(ref name) => println!("Hello {name}!"),
            &None => println!("I don't know who you are."),
        };

        //  matches!
        assert!(matches!('b', 'A'..='Z' | 'a'..='z'));
    }

    // ________________________________________________
    // Box patterns

    {
        fn foo(_value: &Box<i32>) {}

        let k = Some(Box::new(10));

        if let Some(a) = k.as_ref() {
            foo(a);
        }

        if let Some(ref k_ref) = k {
            foo(k_ref);
        }
    }
}
