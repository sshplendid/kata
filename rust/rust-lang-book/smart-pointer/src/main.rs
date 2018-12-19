#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

fn main() {
    store_data_with_box();
    store_cons_list();
}

fn store_data_with_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn store_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
