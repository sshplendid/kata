fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.1, y: 4.1 };

    println!("integer point {:?}", integer);
    println!("float point {:?}", float);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


