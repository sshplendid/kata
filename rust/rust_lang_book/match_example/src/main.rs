#[derive(Debug)]
enum UsState {
    Alabama,
    Ajklaska,
    // ...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let penny = case_value_in_cents(Coin::Penny);
    println!("penny is {} cent", penny);

    let alabama_quarter = case_value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("this quarter is {:?}", alabama_quarter);

    case_option_with_type();
    case_placeholder();
}

fn case_value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn case_option_with_type() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five plus one => {:?}", six);
    println!("none plus one => {:?}", none);
}

fn case_placeholder() {
    let some_u8_val = 1u8;

    match some_u8_val {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
