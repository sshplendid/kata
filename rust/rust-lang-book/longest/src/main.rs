use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    let string1 = &args[1];
    let string2 = &args[2];

    let result = longest(string1.as_str(), string2);

    println!("The longest string is '{}'.", result);
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}
