use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

#[derive(Debug,Clone,Copy)]
struct Portal<T>(T);

impl<T> Portal<T> {
    fn new(val: T) -> Portal<T> {
        Portal(val)
    }
}

impl<T> Deref for Portal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_deref() {
    let p = Portal::new(3);
    assert_eq!(*p, 3);
    // let b = Arc::new(RefCell::new(Portal::new(1)));
    // println!("b = {}",b.borrow())
}

fn print_value(value: &i32) {
    println!("value is {}", value);
}

#[test]
fn test_deref_coercion() {
    let p = Portal::new(3);
    print_value(&p);
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: String::from(name),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[test]
fn test_deref_person() {
    let p = Person::new("A");
    let bp = Box::new(p);
    // let t = bp.get_name();
    println!("{}", bp.get_name())
}

impl<T> DerefMut for Portal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_deref_mut_coercion() {
    let mut p = Portal(7);
    *p = 1;
    println!("{:?}", p);


    let mut p1 = Portal("sdfasdfasf".to_owned());
    p1.push_str("I'm a string");
    println!("{:?}", p1);
}
