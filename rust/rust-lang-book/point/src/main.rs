fn main() {
    test_will_work();
    test_multi_types();
    test_implementation();
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}



fn test_will_work() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.1, y: 4.1 };

    println!("integer point {:?}", integer);
    println!("float point {:?}", float);
}

fn test_multi_types() {
    let integer_and_float = Point { x:1, y: 2.1 };

    println!("{:?}", integer_and_float);
}

fn test_implementation() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let other = Point { x: 1.1, y: 2.2 };

    let mixup = p.mixup(other);

    println!("x of mixed up point is {}", mixup.x());


    let p = Point { x: b'c', y: 10 };
    let other = Point { x: 1.1, y: 2.2 };

    let mixup = p.mixup(other);
    println!("x of mixed up point is {}", mixup.x());

    //println!("{:?}", p);
}
