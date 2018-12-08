extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::calculator::add_two(2));
}
