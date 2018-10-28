fn main() {
    let mut s = String::from("hello world");
    {
        let word = first_word(&s);

        println!("first word is until {}.", word);
    }
    s.clear();

    {
        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[3..];
    }

    {
        let arr = [1, 2, 3, 4, 5];

        let slice1 = &arr[..4];

        for i in slice1 {
            println!("slice1! {}", i);
        }

        let slice2 = &arr[2..];

        for i in slice2 {
            println!("slice2! {}", i);
        }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
