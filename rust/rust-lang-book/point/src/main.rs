fn main() {
    test_will_work();
    test_wont_work();
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


fn test_will_work() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.1, y: 4.1 };

    println!("integer point {:?}", integer);
    println!("float point {:?}", float);
}

fn test_wont_work() {
    let wont_work = Point { x:1, y: 2.1 };

    println!("{:?}", wont_work);
}
