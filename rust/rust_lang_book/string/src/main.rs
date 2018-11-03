fn main() {
    println!("Hello, world!");
    case_string_test();
    create_a_new_string();
    update_a_string();
    concat_with_plus();
    indexing_into_string();
}

fn case_string_test() {
    let s = "Hello World";
    let a = &s[0..1];
    println!("{}", a);

    let s = String::from("Hello World");
    let a: &str = &s[..];
    println!("{}", a);
}

fn create_a_new_string() {
    // creating a new empty string.
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s: String = "initial contents".to_string();

    let s: String = String::from("initial content");
    let hello = String::from("안녕하세요");
    let hello = "안녕하세요".to_string();
}

fn update_a_string() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);
}

fn concat_with_plus() {
    // let s1 = "Hello ";
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // String.add(self, s : &str) -> String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1,s2, s3);
    println!("{}", s);
}

fn indexing_into_string() {
    let s = String::from("hello");
    // let h = s1[0]; // error!

    // inner representation
    
    let len = String::from("Hola").len();
    println!("length 'Hola' => {}", len); // 4
    
    let len = String::from("안녕하세요").len();
    println!("length '안녕하세요' => {}", len); // 4
    
    let s = "안녕하세요";
    println!("{}", &s[0..3]);

    for c in s.chars() {
        println!("char => {}", c);
    }

    for b in s.bytes() {
        println!("byte => {}", b);
    }
}
