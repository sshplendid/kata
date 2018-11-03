#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {      
    // 파라미터를 사용한 함수 호출
    case_area_with_parameters();
    // 튜플로 구조화 한 후 함수 호출
    case_area_with_tuple();
    // 개별 속성 식별 가능한 구조체로 리팩토링
    case_area_with_struct();
    case_method_has_more_parameters();
    case_associate_function();
}


fn case_area_with_parameters() {
    let length = 50;
    let width = 30;
    println!("The area of the rectangle with length={}, width={} is {}", length, width, area(length, width));
}

fn case_area_with_tuple() {
    let length = 50;
    let width = 30;

    let dimension = (length, width);
    println!("The area of the rectangle with tuple {:?} is {}", dimension, area_with_tuple(dimension));
}

fn case_area_with_struct() {
    let length = 50;
    let width = 30;
    let rect = Rectangle {
        length,
        width,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
        );
    
    println!("rect is {:#?}", rect);
}

fn case_associate_function() {
    let size = 6;

    let square = Rectangle::square(size);

    println!("this is square => {:#?}", square);
}

fn case_method_has_more_parameters() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_with_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
