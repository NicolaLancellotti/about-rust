use std::cell::{Cell, LazyCell, OnceCell, RefCell};

/*
   Interior mutability is a design pattern in Rust for allowing you to mutate data even
   though there are immutable references to that data.

   The borrowing rules’ invariants are enforce at runtime
*/

fn take_immutable(_value: &String) {}
fn take_mutable(_value: &mut String) {}

#[test]
fn ref_cell() {
    let p = RefCell::new(String::from("a"));
    {
        take_immutable(&p.borrow());
        take_mutable(&mut p.borrow_mut());
    }

    {
        let mut x1 = p.borrow_mut();
        *x1 = String::from("b");
        // let x2 = p.borrow_mut();
    }

    let b = p.into_inner();
    assert_eq!(b, "b");
}

#[test]
fn cell() {
    let cell = Cell::new("1".to_string());
    let value = cell.replace("2".to_string());
    assert_eq!(value, "1");

    let value = cell.take();
    assert_eq!(value, "2");

    cell.set("3".to_string());
    let value = cell.into_inner();
    assert_eq!(value, "3");
}

#[test]
fn one_cell() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &usize = cell.get_or_init(|| 1);
    assert_eq!(value, &1);
    assert!(cell.get().is_some());
}

#[test]
fn lazy_cell() {
    let cell = LazyCell::new(|| 1);
    assert_eq!(*cell, 1);
}
