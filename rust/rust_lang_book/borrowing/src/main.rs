fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("mutable reference: {}", s);
    }

    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // 스코프 내에서 가변 참조는 한 번만 가능하다.
        // let r2 = &mut s; 
        do_something(r1);
    }

    /**/
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        do_something(r1);
    }
    let r2:&mut String = &mut s;
    do_something(r2);
    //*/
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(some_string: & mut String) {
    // 빌린 값은 기본적으로 불변이다.
    // `mut` 표기가 있어야 변경이 가능하다.
    some_string.push_str(", world"); 
}

fn do_something(a_string: &mut String) {
    a_string.push_str(".");
}
