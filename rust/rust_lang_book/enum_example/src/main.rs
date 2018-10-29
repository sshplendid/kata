#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    case_enum_value();
    case_variant_enums();
}

fn route(ip_type: IpAddrKind) { println!("Go to {:?}", ip_type);}


fn case_enum_value() {
    route(IpAddrKind::V4(10, 240, 159, 91));
    route(IpAddrKind::V6(String::from("::1")));
}

/*
fn case_enum_value_2() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
*/

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
        println!("Message is => {:?}", self);
    }
}

fn case_variant_enums() {
    let greeting = Message::Write(String::from("hello"));
    greeting.call();

    let black = Message::ChangeColor(0, 0, 0);
    black.call();

    let go_left = Message::Move{x: -1, y: 0};
    go_left.call();
}


fn case_option_better_than_null() {
    let some_number = Some(5);
    let some_str = Some("a string");

    let absent_number: Option<i32> = None; 
    // None을 사용하려면 Option<T>가 어떤 타입인지 명시해야 한다.
    // 컴파일러는 None만 보고 어떤 타입인지 추론할 수 없다.
}
