use std::fmt::Debug;

#[derive(Debug)]
enum List<T: Debug> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        println!("Dropping List with data {:?}!", self);
    }
}

use self::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    ref_counter_test();
}

fn ref_counter_test() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a -> {:?}", a);
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("b -> {:?}", b);
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("  c -> {:?}", c);
        println!("  count after creating a = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
