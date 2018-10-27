fn main() {
    // let ref_to_nothing = dangle();

    let safe = no_dangle();
}

/*
// scope에서 사라진 메모리 주소를 리턴한다. 컴파일러에서 오류 발생
fn dangle() -> &String {
    let s = String::from("hello");
    
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
