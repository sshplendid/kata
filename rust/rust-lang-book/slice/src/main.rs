fn main() {
    slice_1();
    slice_test();
    string_test();
}

fn slice_1() {
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

fn slice_test() {
    let numbers: [i32; 3] = [1, 2, 3];

    assert_eq!([1, 2, 3], &numbers[..]);

    for n in numbers.iter() {
        println!("number! {}", n);
    }
}

fn string_test() {
    let str1 = "hello";
    let str2: &str = "world";
    let str3: String = String::from("Rust is awesome!");
    let str4: &str = &str3[2..8];
    let str5: &str = &str2[2..5];
    println!("str1=>{}", str1);
    println!("str2=>{}", str2);
    println!("str3=>{}", str3);
    println!("str4=>{}", str4);
    println!("str5=>{}", str5);
}
