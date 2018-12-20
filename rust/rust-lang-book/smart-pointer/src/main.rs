#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    store_data_with_box();
    println!("---------");
    store_cons_list();
    println!("---------");
    get_value_with_deref();
    println!("---------");
    get_value_with_box();
    println!("---------");
    mybox();
}

fn store_data_with_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn store_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

fn get_value_with_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("deref y = {}", *y);
    println!("ref y = {}", y);
}

fn get_value_with_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("deref y = {}", *y);
    println!("ref y = {}", y);
}

fn mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
