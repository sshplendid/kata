#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let length = 50;
    let width = 30;

    let rect = Rectangle {
        length,
        width,
    };

    println!("rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(rect)
        );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_with_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_with_struct(rect: Rectangle) -> u32 {
    rect.length * rect.width
}
