fn main() {
    case_vector1();
    case_vector2();
    case_vector3();
    case_vector4();
    case_vector_read();
    println!("Hello, world!");
}

fn case_vector1() {
    let v: Vec<i32> = Vec::new(); // 불변의 벡터, 타입을 명시해야 함.
                                  // v.push(5); // ! error

    println!("v => {:?}", v);
}

fn case_vector2() {
    let mut v = Vec::new(); // 아래 코드에서 데이터로부터 타입을 추론하므로 타입을 명시할 필요가 없다.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v => {:?}", v);
}

fn case_vector3() {
    let v = vec![1, 2, 3]; // 초기값으로부터 타입을 추론한다.

    println!("v => {:?}", v);
}

fn case_vector4() {
    {
        let v: Vec<&str> = vec!["a", "b", "c", "d"];
        // do something with v
        println!("v => {:?}", v);
    }
    // v is dropped.
}

fn case_vector_read() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[3]; // 벡터 길이를 초과하는 요소를 조회하면 panic!
    println!("third => {}", third);
    let third: Option<&i32> = v.get(8); // n%size로 조회
    match third {
        Some(n) => {
            println!("third => {}", n);
        }
        _ => {
            println!("There's no element!");
        }
    }
}
