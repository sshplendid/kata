fn main() {
    store_data_with_box();
    println!("Hello, world!");
}

fn store_data_with_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}
