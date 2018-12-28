use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    visualize(&leaf, "leaf");

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch = {:?}", branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        visualize(&branch, "branch");
        visualize(&leaf, "leaf");
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    visualize(&leaf, "leaf");
}

fn visualize(node: &Rc<Node>, name: &str) {
    println!(
        "{} strong = {}, weak = {}",
        name,
        Rc::strong_count(&node),
        Rc::weak_count(&node)
    );
}
