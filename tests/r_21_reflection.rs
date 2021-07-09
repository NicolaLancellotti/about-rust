use std::any::Any;
use std::any::TypeId;
use std::boxed::Box;
use std::mem;

#[test]
fn reflection() {
    {
        let t = TypeId::of::<i32>();
        println!("{t:?}");
    }

    {
        let mut value = 10;
        let any_value: &mut dyn Any = &mut value;
        assert!(any_value.is::<i32>());

        match any_value.downcast_ref::<i32>() {
            Some(v) => {
                assert_eq!(v, &10);
            }
            None => (),
        }

        match any_value.downcast_mut::<i32>() {
            Some(v) => {
                *v = 11;
            }
            None => (),
        }
        assert_eq!(value, 11);
    }

    {
        let box1: Box<dyn Any> = Box::new(10);
        match box1.downcast::<i32>() {
            Ok(v) => {
                assert_eq!(*v, 10);
            }
            Err(_) => (),
        }
    }

    {
        println!("Instance of {}", std::any::type_name::<i32>());
        println!("Instance of {}", std::any::type_name_of_val(&1));
    }
}

#[test]
fn offset() {
    struct Struct {
        a: u16,
        b: u8,
    }
    assert_eq!(mem::offset_of!(Struct, b), 2);
}
