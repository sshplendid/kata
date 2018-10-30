fn main() {
    case_match();
    case_if_let();
}

fn case_match() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three!"),
        _ => (),
    }
}

fn case_if_let() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    }
}
